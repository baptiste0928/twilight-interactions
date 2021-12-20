use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, FieldsNamed, Result};

use crate::{
    command::model::parse::{channel_type, command_option_value},
    parse::find_attr,
};

use super::parse::{FieldType, StructField, TypeAttribute};

/// Implementation of CommandModel derive macro
pub fn impl_command_model(input: DeriveInput, fields: Option<FieldsNamed>) -> Result<TokenStream> {
    let ident = &input.ident;
    let fields = match fields {
        Some(fields) => StructField::from_fields(fields)?,
        None => Vec::new(),
    };

    let partial = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?.partial,
        None => false,
    };

    let field_unknown = field_unknown(partial);
    let fields_init = fields.iter().map(field_init);
    let fields_match_arms = fields.iter().map(field_match_arm);
    let fields_constructor = fields.iter().map(field_constructor);

    Ok(quote! {
        impl ::twilight_interactions::command::CommandModel for #ident {
            fn from_interaction(
                data: ::twilight_interactions::command::CommandInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                #(#fields_init)*

                for opt in data.options {
                    match &*opt.name {
                        #(#fields_match_arms,)*
                        other => #field_unknown
                    }
                }

                ::std::result::Result::Ok(Self { #(#fields_constructor),* })
            }
        }
    })
}

/// Generate field initialization variables
fn field_init(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    quote!(let mut #ident = None;)
}

/// Generate field match arm
fn field_match_arm(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let span = field.span;

    let name = field.attributes.name_default(ident.to_string());
    let channel_types = field.attributes.channel_types.iter().map(channel_type);
    let max_value = command_option_value(field.attributes.max_value);
    let min_value = command_option_value(field.attributes.min_value);

    quote_spanned! {span=>
        #name => {
            let option_data = ::twilight_interactions::command::internal::CommandOptionData {
                channel_types: ::std::vec![#(#channel_types),*],
                max_value: #max_value,
                min_value: #min_value,
            };

            match ::twilight_interactions::command::CommandOption::from_option(opt.value, option_data, data.resolved.as_deref()) {
                ::std::result::Result::Ok(value) => #ident = Some(value),
                ::std::result::Result::Err(kind) => {
                    return ::std::result::Result::Err(
                        ::twilight_interactions::error::ParseError::Option(
                            ::twilight_interactions::error::ParseOptionError {
                                field: ::std::convert::From::from(#name),
                                kind,
                        })
                    )
                }
            }
        }
    }
}

/// Generate field constructor
fn field_constructor(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let ident_str = ident.to_string();

    match field.kind {
        FieldType::Required => quote! {
            #ident: match #ident {
                Some(value) => value,
                None => return Err(::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(#ident_str),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::RequiredField
                }))
            }
        },
        FieldType::Optional => quote!(#ident),
    }
}

/// Generate unknown field match arm
fn field_unknown(partial: bool) -> TokenStream {
    if partial {
        quote!(continue)
    } else {
        quote! {
            return ::std::result::Result::Err(
                ::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(other),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::UnknownField,
                })
            )
        }
    }
}

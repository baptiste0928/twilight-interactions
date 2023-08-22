use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Error, FieldsNamed, Result};

use super::parse::{FieldType, StructField, TypeAttribute};
use crate::{
    command::model::parse::{channel_type, command_option_value},
    parse::syntax::{find_attr, optional},
};

/// Implementation of `CommandModel` derive macro
pub fn impl_command_model(input: DeriveInput, fields: Option<FieldsNamed>) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let fields = match fields {
        Some(fields) => StructField::from_fields(fields)?,
        None => Vec::new(),
    };

    let autocomplete = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?.autocomplete.unwrap_or(false),
        None => false,
    };

    for field in &fields {
        // If autocomplete, ensure all fields are either `AutocompleteValue` or `Option`s
        if autocomplete && ![FieldType::Autocomplete, FieldType::Optional].contains(&field.kind) {
            return Err(Error::new(
                field.span,
                "autocomplete models only supports `Option` or `AutocompleteValue` field type",
            ));
        }

        // `AutocompleteValue` is only allowed in autocomplete models
        if !autocomplete && field.kind == FieldType::Autocomplete {
            return Err(Error::new(
                field.span,
                "`AutocompleteValue` is only available in autocomplete models, add the `#[command(autocomplete = true)]` attribute to the type"
            ));
        }
    }

    let field_unknown = field_unknown(autocomplete);
    let fields_init = fields.iter().map(field_init);
    let fields_match_arms = fields.iter().map(field_match_arm);
    let fields_constructor = fields.iter().map(field_constructor);

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CommandModel for #ident #generics #where_clause {
            fn from_interaction(
                __data: ::twilight_interactions::command::CommandInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                #(#fields_init)*

                for __opt in __data.options {
                    match &*__opt.name {
                        #(#fields_match_arms,)*
                        __other => #field_unknown
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
    let max_value = command_option_value(field.attributes.max_value);
    let min_value = command_option_value(field.attributes.min_value);
    let max_length = optional(field.attributes.max_length);
    let min_length = optional(field.attributes.min_length);

    let channel_types = if field.attributes.channel_types.is_empty() {
        quote! { ::std::option::Option::None }
    } else {
        let items = field.attributes.channel_types.iter().map(channel_type);
        quote! { ::std::option::Option::Some(::std::vec![#(#items),*]) }
    };

    quote_spanned! {span=>
        #name => {
            let __option_data = ::twilight_interactions::command::internal::CommandOptionData {
                channel_types: #channel_types,
                max_value: #max_value,
                min_value: #min_value,
                max_length: #max_length,
                min_length: #min_length,
            };

            match ::twilight_interactions::command::CommandOption::from_option(__opt.value, __option_data, __data.resolved.as_deref()) {
                ::std::result::Result::Ok(__value) => #ident = Some(__value),
                ::std::result::Result::Err(__kind) => {
                    return ::std::result::Result::Err(
                        ::twilight_interactions::error::ParseError::Option(
                            ::twilight_interactions::error::ParseOptionError {
                                field: ::std::convert::From::from(#name),
                                kind: __kind,
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
                Some(__value) => __value,
                None => return Err(::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(#ident_str),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::RequiredField
                }))
            }
        },
        FieldType::Optional => quote!(#ident),
        FieldType::Autocomplete => quote! {
            #ident: match #ident {
                Some(__value) => __value,
                None => ::twilight_interactions::command::AutocompleteValue::None,
            }
        },
    }
}

/// Generate unknown field match arm
fn field_unknown(autocomplete: bool) -> TokenStream {
    if autocomplete {
        quote!(continue)
    } else {
        quote! {
            return ::std::result::Result::Err(
                ::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(__other),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::UnknownField,
                })
            )
        }
    }
}

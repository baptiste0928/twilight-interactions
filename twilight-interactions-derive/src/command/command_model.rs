use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields, Ident, Result};

use crate::{command::attributes::TypeAttribute, parse::find_attr};

use super::fields::{FieldType, StructField};

/// Implementation of CommandModel derive macro
pub fn impl_command_model(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    // Parse type fields
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => StructField::from_fields(fields)?,
        _ => {
            return Err(Error::new(
                input.span(),
                "`#[derive(CommandModel)] can only be applied to structs with named fields",
            ));
        }
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
                data: ::twilight_model::application::interaction::application_command::CommandData,
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

/// Dummy implementation of the `CommandModel` trait in case of macro error
pub fn dummy_command_model(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::command::CommandModel for #ident {
            fn from_interaction(
                data: ::twilight_model::application::interaction::application_command::CommandData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                ::std::unimplemented!()
            }
        }
    }
}

/// Generate field initialization variables
fn field_init(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    quote!(let mut #ident = None;)
}

/// Generate field match arm
fn field_match_arm(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let name = field.attributes.name_default(ident.to_string());
    let span = field.span;

    quote_spanned! {span=>
        #name => match ::twilight_interactions::command::CommandOption::from_option(opt.value, data.resolved.as_ref()) {
            ::std::result::Result::Ok(value) => #ident = Some(value),
            ::std::result::Result::Err(kind) => {
                return ::std::result::Result::Err(
                    ::twilight_interactions::error::ParseError {
                        field: ::std::convert::From::from(#name),
                        kind,
                    }
                )
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
                None => return Err(::twilight_interactions::error::ParseError {
                    field: ::std::convert::From::from(#ident_str),
                    kind: ::twilight_interactions::error::ParseErrorType::RequiredField
                })
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
                ::twilight_interactions::error::ParseError {
                    field: ::std::convert::From::from(other),
                    kind: ::twilight_interactions::error::ParseErrorType::UnknownField,
                }
            )
        }
    }
}

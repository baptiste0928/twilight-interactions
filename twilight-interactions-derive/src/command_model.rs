use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataStruct, DeriveInput, Error, Fields, Result};

use crate::attributes::{find_attr, FieldAttributes};

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

    let fields_init = fields.iter().map(field_init);
    let fields_match_arms = fields.iter().map(field_match_arm);
    let fields_constructor = fields.iter().map(field_constructor);

    Ok(quote! {
        impl ::twilight_interactions::CommandModel for #ident {
            fn from_interaction(
                data: ::twilight_model::application::interaction::application_command::CommandData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                #(#fields_init)*

                for opt in data.options {
                    match &*opt.name {
                        #(#fields_match_arms,)*
                        other => {
                            return ::std::result::Result::Err(
                                ::twilight_interactions::error::ParseError {
                                    field: ::std::convert::From::from(other),
                                    kind: ::twilight_interactions::error::ParseErrorType::UnknownField,
                                }
                            )
                        }
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
    let name = field
        .attributes
        .rename
        .clone()
        .unwrap_or_else(|| ident.to_string());
    let span = field.span;

    quote_spanned! {span=>
        #name => match ::twilight_interactions::CommandOption::from_option(opt.value, data.resolved.as_ref()) {
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

/// Parsed struct field
struct StructField {
    span: Span,
    ident: Ident,
    attributes: FieldAttributes,
    kind: FieldType,
}

/// Type of a parsed struct field
enum FieldType {
    Required,
    Optional,
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    fn from_field(field: syn::Field) -> Result<Self> {
        let kind = match crate::extract_option(&field.ty) {
            Some(_) => FieldType::Optional,
            None => FieldType::Required,
        };

        let attributes = match find_attr(&field.attrs, "command") {
            Some(attr) => FieldAttributes::parse(attr)?,
            None => FieldAttributes::default(),
        };

        Ok(Self {
            span: field.span(),
            ident: field.ident.unwrap(),
            attributes,
            kind,
        })
    }

    /// Parse [`syn::FieldsNamed`] as a [`Vec<StructField>`]
    fn from_fields(fields: syn::FieldsNamed) -> Result<Vec<Self>> {
        fields.named.into_iter().map(Self::from_field).collect()
    }
}

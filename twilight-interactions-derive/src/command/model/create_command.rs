use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, FieldsNamed, Result};

use crate::parse::{find_attr, parse_doc};

use super::parse::{channel_type, command_option_value, StructField, TypeAttribute};

/// Implementation of CreateCommand derive macro
pub fn impl_create_command(input: DeriveInput, fields: FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let span = input.span();
    let fields = StructField::from_fields(fields)?;

    check_fields_order(&fields)?;

    let capacity = fields.len();
    let (attributes, attr_span) = match find_attr(&input.attrs, "command") {
        Some(attr) => (TypeAttribute::parse(attr)?, attr.span()),
        None => {
            return Err(Error::new(
                span,
                "Missing required #[command(...)] attribute",
            ))
        }
    };

    if attributes.partial {
        return Err(Error::new(
            attr_span,
            "Cannot implement `CreateCommand` on partial model",
        ));
    }

    let name = match &attributes.name {
        Some(name) => name,
        None => return Err(Error::new(attr_span, "Missing required attribute `name`")),
    };
    let description = match &attributes.desc {
        Some(desc) => desc.clone(),
        None => parse_doc(&input.attrs, span)?,
    };
    let default_permission = attributes.default_permission;

    let field_options = fields
        .iter()
        .map(field_option)
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl ::twilight_interactions::command::CreateCommand for #ident {
            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let mut command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#field_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    description: ::std::convert::From::from(#description),
                    options: command_options,
                    default_permission: #default_permission,
                    group: false,
                }
            }
        }
    })
}

/// Generate field option code
fn field_option(field: &StructField) -> Result<TokenStream> {
    let ty = &field.ty;
    let span = field.span;

    let name = field.attributes.name_default(field.ident.to_string());
    let description = match &field.attributes.desc {
        Some(desc) => desc.clone(),
        None => parse_doc(&field.raw_attrs, field.span)?,
    };
    let required = field.kind.required();
    let autocomplete = field.attributes.autocomplete;
    let channel_types = field.attributes.channel_types.iter().map(channel_type);
    let max_value = command_option_value(field.attributes.max_value);
    let min_value = command_option_value(field.attributes.min_value);

    Ok(quote_spanned! {span=>
        command_options.push(<#ty as ::twilight_interactions::command::CreateOption>::create_option(
            ::twilight_interactions::command::internal::CreateOptionData {
                name: ::std::convert::From::from(#name),
                description: ::std::convert::From::from(#description),
                required: #required,
                autocomplete: #autocomplete,
                data: ::twilight_interactions::command::internal::CommandOptionData {
                    channel_types: ::std::vec![#(#channel_types),*],
                    max_value: #max_value,
                    min_value: #min_value,
                },
            }
        ));
    })
}

/// Ensure optional options are after required ones
fn check_fields_order(fields: &[StructField]) -> Result<()> {
    let mut optional_option_added = false;

    for field in fields {
        if !optional_option_added && !field.kind.required() {
            optional_option_added = true;
        }

        if optional_option_added && field.kind.required() {
            return Err(Error::new(
                field.span,
                "Required options should be added before optional",
            ));
        }
    }

    Ok(())
}

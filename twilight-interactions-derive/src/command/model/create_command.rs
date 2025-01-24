use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, FieldsNamed, Result};

use super::parse::{channel_type, command_option_value, StructField, TypeAttribute};
use crate::{
    command::user_application::{context, integration_type},
    localization::{description_expr, name_expr},
    parse::syntax::{find_attr, optional, parse_doc},
};

/// Implementation of `CreateCommand` derive macro
pub fn impl_create_command(input: DeriveInput, fields: Option<FieldsNamed>) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let fields = match fields {
        Some(fields) => StructField::from_fields(fields)?,
        None => Vec::new(),
    };

    check_fields_order(&fields)?;

    let capacity = fields.len();
    let (attributes, attr_span) = match find_attr(&input.attrs, "command") {
        Some(attr) => (TypeAttribute::parse(attr)?, attr.span()),
        None => {
            return Err(Error::new_spanned(
                input,
                "missing required #[command(...)] attribute",
            ))
        }
    };

    if attributes.autocomplete == Some(true) {
        return Err(Error::new(
            attr_span,
            "cannot implement `CreateCommand` on partial model",
        ));
    }

    let name = match attributes.name {
        Some(name) => String::from(name),
        None => return Err(Error::new(attr_span, "missing required attribute `name`")),
    };

    let name_expr = name_expr(&name, &attributes.name_localizations);
    let desc_expr = description_expr(&attributes.desc, &attributes.desc_localizations, || {
        parse_doc(&input.attrs, input.span())
    })?;

    let default_permissions = match &attributes.default_permissions {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };
    let dm_permission = optional(attributes.dm_permission);
    let nsfw = optional(attributes.nsfw);

    let field_options = fields
        .iter()
        .map(field_option)
        .collect::<Result<Vec<_>>>()?;

    let contexts = if let Some(items) = attributes.contexts {
        let items = items.iter().map(context);
        quote! { ::std::option::Option::Some(::std::vec![#(#items),*]) }
    } else {
        quote! { ::std::option::Option::None }
    };

    let integration_types = if let Some(items) = attributes.integration_types {
        let items = items.iter().map(integration_type);
        quote! { ::std::option::Option::Some(::std::vec![#(#items),*]) }
    } else {
        quote! { ::std::option::Option::None }
    };

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CreateCommand for #ident #generics #where_clause {
            const NAME: &'static str = #name;

            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let mut __command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#field_options)*

                let __command_name = #name_expr;
                let __command_desc = #desc_expr;

                ::twilight_interactions::command::ApplicationCommandData {
                    name: __command_name.fallback,
                    name_localizations: __command_name.localizations,
                    description: __command_desc.fallback,
                    description_localizations: __command_desc.localizations,
                    options: __command_options,
                    default_member_permissions: #default_permissions,
                    dm_permission: #dm_permission,
                    nsfw: #nsfw,
                    group: false,
                    contexts: #contexts,
                    integration_types: #integration_types,
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
    let name_expr = name_expr(&name, &field.attributes.name_localizations);

    let desc_expr = description_expr(
        &field.attributes.desc,
        &field.attributes.desc_localizations,
        || parse_doc(&field.raw_attrs, span),
    )?;

    let required = field.kind.required();
    let autocomplete = field.attributes.autocomplete;
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

    Ok(quote_spanned! {span => {
        let __field_desc = #desc_expr;
        let __field_name = #name_expr;

        __command_options.push(<#ty as ::twilight_interactions::command::CreateOption>::create_option(
            ::twilight_interactions::command::internal::CreateOptionData {
                name: __field_name.fallback,
                name_localizations: __field_name.localizations,
                description: __field_desc.fallback,
                description_localizations: __field_desc.localizations,
                required: ::std::option::Option::Some(#required),
                autocomplete: #autocomplete,
                data: ::twilight_interactions::command::internal::CommandOptionData {
                    channel_types: #channel_types,
                    max_value: #max_value,
                    min_value: #min_value,
                    max_length: #max_length,
                    min_length: #min_length,
                },
            }
        ));
    }})
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
                "required options should be added before optional",
            ));
        }
    }

    Ok(())
}

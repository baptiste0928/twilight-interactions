use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, FieldsNamed, Result};

use super::parse::{channel_type, command_option_value, optional, StructField, TypeAttribute};
use crate::parse::{find_attr, parse_doc};

/// Implementation of `CreateCommand` derive macro
pub fn impl_create_command(input: DeriveInput, fields: Option<FieldsNamed>) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let span = input.span();
    let fields = match fields {
        Some(fields) => StructField::from_fields(fields)?,
        None => Vec::new(),
    };

    check_fields_order(&fields)?;

    let capacity = fields.len();
    let (attributes, attr_span) = match find_attr(&input.attrs, "command") {
        Some(attr) => (TypeAttribute::parse(attr)?, attr.span()),
        None => {
            return Err(Error::new(
                span,
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

    let name = match &attributes.name {
        Some(name) => name,
        None => return Err(Error::new(attr_span, "missing required attribute `name`")),
    };
    let name_localizations = localization_field(&attributes.name_localizations);
    let description = match &attributes.desc {
        Some(desc) => quote! { ::std::option::Option::Some((#desc).to_string()) },
        None => match parse_doc(&input.attrs, span).ok() {
            Some(doc) => quote! { ::std::option::Option::Some((#doc).to_string()) },
            None => quote! { ::std::option::Option::None },
        },
    };
    let description_localizations = localization_field(&attributes.desc_localizations);
    let default_permissions = match &attributes.default_permissions {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };
    let dm_permission = match &attributes.dm_permission {
        Some(dm_permission) => quote! { ::std::option::Option::Some(#dm_permission)},
        None => quote! { ::std::option::Option::None },
    };
    let nsfw = match &attributes.nsfw {
        Some(nsfw) => quote! { ::std::option::Option::Some(#nsfw) },
        None => quote! { ::std::option::Option::None },
    };

    let field_options = fields
        .iter()
        .map(field_option)
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CreateCommand for #ident #generics #where_clause {
            const NAME: &'static str = #name;

            fn create_command() -> ::std::result::Result<
                ::twilight_interactions::command::ApplicationCommandData,
                ::twilight_interactions::error::CreateCommandError
            > {
                let mut command_options = ::std::vec::Vec::with_capacity(#capacity);
                #(#field_options)*

                let mut desc_locales: ::std::option::Option<::std::collections::HashMap<String, String>> = #description_localizations;
                let description = match #description {
                    ::std::option::Option::Some(val) => val,
                    ::std::option::Option::None => {
                        let ::std::option::Option::Some(desc_locales) = desc_locales.as_mut() else {
                            return ::std::result::Result::Err(
                                ::twilight_interactions::error::CreateCommandError::NoCommandDescription(#name)
                            );
                        };

                        let ::std::option::Option::Some(desc) = desc_locales.remove("") else {
                            return ::std::result::Result::Err(
                                ::twilight_interactions::error::CreateCommandError::NoCommandDescription(#name)
                            );
                        };

                        desc
                    },
                };

                ::std::result::Result::Ok(::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    name_localizations: #name_localizations,
                    description,
                    description_localizations: desc_locales,
                    options: command_options,
                    default_member_permissions: #default_permissions,
                    dm_permission: #dm_permission,
                    nsfw: #nsfw,
                    group: false,
                })
            }
        }
    })
}

/// Generate field option code
fn field_option(field: &StructField) -> Result<TokenStream> {
    let ty = &field.ty;
    let span = field.span;

    let name = field.attributes.name_default(field.ident.to_string());
    let name_localizations = localization_field(&field.attributes.name_localizations);
    let description = match &field.attributes.desc {
        Some(desc) => quote! { ::std::option::Option::Some((#desc).to_string()) },
        None => match parse_doc(&field.raw_attrs, field.span).ok() {
            Some(doc) => quote! { ::std::option::Option::Some((#doc).to_string()) },
            None => quote! { ::std::option::Option::None },
        },
    };
    // let description = match &field.attributes.desc {
    //     Some(desc) => desc.clone(),
    //     None => parse_doc(&field.raw_attrs, field.span)?,
    // };
    let description_localizations = localization_field(&field.attributes.desc_localizations);
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

    Ok(quote_spanned! {span=>
        let mut desc_locales: ::std::option::Option<::std::collections::HashMap<String, String>> = #description_localizations;
        let description = match #description {
            ::std::option::Option::Some(val) => val,
            ::std::option::Option::None => {
                let ::std::option::Option::Some(desc_locales) = desc_locales.as_mut() else {
                    return ::std::result::Result::Err(
                        ::twilight_interactions::error::CreateCommandError::NoOptionDescription(#name)
                    );
                };

                let ::std::option::Option::Some(desc) = desc_locales.remove("") else {
                    return ::std::result::Result::Err(
                        ::twilight_interactions::error::CreateCommandError::NoOptionDescription(#name)
                    );
                };

                desc
            },
        };

        command_options.push(<#ty as ::twilight_interactions::command::CreateOption>::create_option(
            ::twilight_interactions::command::internal::CreateOptionData {
                name: ::std::convert::From::from(#name),
                name_localizations: #name_localizations,
                description,
                description_localizations: desc_locales,
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
    })
}

fn localization_field(path: &Option<syn::Path>) -> TokenStream {
    match path {
        Some(path) => {
            quote! {
                ::std::option::Option::Some(
                    ::twilight_interactions::command::internal::convert_localizations(#path())
                )
            }
        }
        None => quote! { ::std::option::Option::None },
    }
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

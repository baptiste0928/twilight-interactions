use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, Result, Variant};

use super::parse::{ParsedVariant, TypeAttribute};
use crate::{
    command::user_application::{context, integration_type},
    localization::{description_expr, name_expr},
    parse::syntax::{find_attr, optional, parse_doc},
};

/// Implementation of `CreateCommand` derive macro
pub fn impl_create_command(
    input: DeriveInput,
    variants: impl IntoIterator<Item = Variant>,
) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;

    let variants = ParsedVariant::from_variants(variants, input.span())?;
    let attributes = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?,
        None => {
            return Err(Error::new_spanned(
                input,
                "missing required #[command(...)] attribute",
            ))
        }
    };

    let name = String::from(attributes.name);
    let name_expr = name_expr(&name, &attributes.name_localizations);

    let desc_expr = description_expr(&attributes.desc, &attributes.desc_localizations, || {
        parse_doc(&input.attrs, input.span())
    })?;

    let capacity = variants.len();
    let default_permissions = match &attributes.default_permissions {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };
    let dm_permission = optional(attributes.dm_permission);
    let nsfw = optional(attributes.nsfw);

    let variant_options = variants.iter().map(variant_option);

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
                let __command_name = #name_expr;
                let __command_desc = #desc_expr;
                let mut __command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#variant_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: __command_name.fallback,
                    name_localizations: __command_name.localizations,
                    description: __command_desc.fallback,
                    description_localizations: __command_desc.localizations,
                    options: __command_options,
                    default_member_permissions: #default_permissions,
                    dm_permission: #dm_permission,
                    nsfw: #nsfw,
                    group: true,
                    contexts: #contexts,
                    integration_types: #integration_types,
                }
            }
        }
    })
}

/// Generate variant option code
fn variant_option(variant: &ParsedVariant) -> TokenStream {
    let ty = &variant.inner;
    let span = variant.span;

    quote_spanned! {span=>
        __command_options.push(::std::convert::From::from(
            <#ty as ::twilight_interactions::command::CreateCommand>::create_command()
        ));
    }
}

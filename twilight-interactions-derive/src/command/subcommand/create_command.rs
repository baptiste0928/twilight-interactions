use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, Result, Variant};

use super::parse::{ParsedVariant, TypeAttribute};
use crate::{
    command::description::get_description,
    parse::{
        parsers::FunctionPath,
        syntax::{find_attr, optional},
    },
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
    let attribute = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?,
        None => {
            return Err(Error::new_spanned(
                input,
                "missing required #[command(...)] attribute",
            ))
        }
    };

    let desc = get_description(
        &attribute.desc_localizations,
        &attribute.desc,
        input.span(),
        &input.attrs,
    )?;

    let capacity = variants.len();
    let name = &attribute.name;
    let name_localizations = localization_field(&attribute.name_localizations);
    let default_permissions = match &attribute.default_permissions {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };
    let dm_permission = optional(attribute.dm_permission);
    let nsfw = optional(attribute.nsfw);

    let variant_options = variants.iter().map(variant_option);

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CreateCommand for #ident #generics #where_clause {
            const NAME: &'static str = #name;

            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let desc = #desc;
                let mut __command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#variant_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    name_localizations: #name_localizations,
                    description: desc.0,
                    description_localizations: desc.1,
                    options: __command_options,
                    default_member_permissions: #default_permissions,
                    dm_permission: #dm_permission,
                    nsfw: #nsfw,
                    group: true,
                }
            }
        }
    })
}

fn localization_field(path: &Option<FunctionPath>) -> TokenStream {
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

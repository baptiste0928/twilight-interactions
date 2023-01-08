use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, Result, Variant};

use super::parse::{ParsedVariant, TypeAttribute};
use crate::parse::{find_attr, parse_doc};

/// Implementation of `CreateCommand` derive macro
pub fn impl_create_command(
    input: DeriveInput,
    variants: impl IntoIterator<Item = Variant>,
) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let span = input.span();

    let variants = ParsedVariant::from_variants(variants, input.span())?;
    let attribute = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?,
        None => {
            return Err(Error::new(
                span,
                "Missing required #[command(...)] attribute",
            ))
        }
    };

    let capacity = variants.len();
    let name = &attribute.name;
    let name_localizations = localization_field(&attribute.name_localizations);
    let description_localizations = localization_field(&attribute.desc_localizations);
    let description = match attribute.desc {
        Some(desc) => desc,
        None => parse_doc(&input.attrs, span)?,
    };
    let default_permissions = match &attribute.default_permissions {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };
    let dm_permission = match &attribute.dm_permission {
        Some(dm_permission) => quote! { ::std::option::Option::Some(#dm_permission)},
        None => quote! { ::std::option::Option::None },
    };
    let nsfw = match &attribute.nsfw {
        Some(nsfw) => quote! { ::std::option::Option::Some(#nsfw) },
        None => quote! { std::option::Option::None },
    };

    let variant_options = variants.iter().map(variant_option);

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CreateCommand for #ident #generics #where_clause {
            const NAME: &'static str = #name;

            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let mut command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#variant_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    name_localizations: #name_localizations,
                    description: ::std::convert::From::from(#description),
                    description_localizations: #description_localizations,
                    options: command_options,
                    default_member_permissions: #default_permissions,
                    dm_permission: #dm_permission,
                    nsfw: #nsfw,
                    group: true,
                }
            }
        }
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

/// Generate variant option code
fn variant_option(variant: &ParsedVariant) -> TokenStream {
    let ty = &variant.inner;
    let span = variant.span;

    quote_spanned! {span=>
        command_options.push(::std::convert::From::from(
            <#ty as ::twilight_interactions::command::CreateCommand>::create_command()
        ));
    }
}

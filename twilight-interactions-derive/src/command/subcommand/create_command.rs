use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Error, Result, Variant};

use crate::parse::{find_attr, parse_doc};

use super::parse::{ParsedVariant, TypeAttribute};

/// Implementation of CreateCommand derive macro
pub fn impl_create_command(
    input: DeriveInput,
    variants: impl IntoIterator<Item = Variant>,
) -> Result<TokenStream> {
    let ident = &input.ident;
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
    let default_permission = attribute.default_permission;
    let description = match attribute.desc {
        Some(desc) => desc,
        None => parse_doc(&input.attrs, span)?,
    };

    let variant_options = variants.iter().map(variant_option);

    Ok(quote! {
        impl ::twilight_interactions::command::CreateCommand for #ident {
            const NAME: &'static str = #name;

            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let mut command_options = ::std::vec::Vec::with_capacity(#capacity);

                #(#variant_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    description: ::std::convert::From::from(#description),
                    options: command_options,
                    default_permission: #default_permission,
                    group: true,
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
        command_options.push(::std::convert::From::from(
            <#ty as ::twilight_interactions::command::CreateCommand>::create_command()
        ));
    }
}

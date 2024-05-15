use crate::parse::parsers::{CommandDescription, FunctionPath};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Error, Result};

/// Parse the description and localizations from the command attributes.
///
/// Description can be specified using the `desc` or `desc_localizations`
/// attributes, which are mutually exclusive.
///
/// If no description is found, the documentation comment is parsed from the
/// item attributes.
pub fn description_expr(
    desc: &Option<CommandDescription>,
    localizations: &Option<FunctionPath>,
    default: impl FnOnce() -> Result<String>,
) -> Result<TokenStream> {
    let localizations_span = localizations.span();

    let description = match (desc, localizations) {
        (Some(desc), None) => desc.to_token_stream(),
        (None, Some(path)) => quote! { #path()},
        (None, None) => default()?.to_token_stream(),
        (Some(_), Some(_)) => {
            return Err(Error::new(
                localizations_span,
                "`desc` and `desc_localizations` are mutually exclusive",
            ))
        }
    };

    Ok(quote_spanned! { localizations_span =>
        ::twilight_interactions::command::internal::IntoLocalizationsInternal::into_localizations(#description)
    })
}

pub fn name_expr(name: &str, name_localizations: &Option<FunctionPath>) -> TokenStream {
    let localizations_span = name_localizations.span();
    let name_localizations = match name_localizations {
        Some(path) => quote! { ::std::option::Option::Some(#path())},
        None => quote! { ::std::option::Option::None },
    };

    quote_spanned! { localizations_span =>
        ::twilight_interactions::command::internal::IntoLocalizationsInternal::into_localizations((#name, #name_localizations))
    }
}

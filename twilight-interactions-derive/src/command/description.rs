use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Result};

use crate::parse::{
    parsers::{CommandDescription, FunctionPath},
    syntax::parse_doc,
};

pub fn get_description(
    desc_localizations: &Option<FunctionPath>,
    desc: &Option<CommandDescription>,
    span: Span,
    attrs: &[Attribute],
) -> Result<TokenStream> {
    if desc.is_some() && desc_localizations.is_some() {
        return Err(syn::Error::new(
            span,
            "You can't specify `desc` and `desc_localizations`.",
        ));
    }

    let desc = match desc_localizations {
        Some(path) => quote! {
            {
                let desc = #path();
                (desc.fallback, ::std::option::Option::Some(desc.localizations))
            }
        },
        None => {
            let desc = match desc {
                Some(desc) => desc.clone().into(),
                None => parse_doc(attrs, span)?,
            };

            quote! { (::std::convert::From::from(#desc), None) }
        }
    };

    Ok(desc)
}

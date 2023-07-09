use quote::quote;

use crate::parse::parse_doc;

pub fn get_description(
    desc_localizations: &Option<syn::Path>,
    desc: &Option<String>,
    span: proc_macro2::Span,
    attrs: &[syn::Attribute],
) -> syn::Result<proc_macro2::TokenStream> {
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
                Some(desc) => desc.clone(),
                None => parse_doc(attrs, span)?,
            };

            quote! { (::std::convert::From::from(#desc), None) }
        }
    };

    Ok(desc)
}

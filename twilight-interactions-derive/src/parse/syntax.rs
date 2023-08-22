//! Rust syntax parsing helpers.

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Error, Expr, GenericArgument, Lit, PathArguments, Result};

/// Find the first attribute with a specific name.
pub fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident(name))
}

/// Extract generic type from a specific type.
///
/// For example, `extract_generic(parse_quote!(Option<String>), "Option")`
/// returns `Some(parse_quote!(String))`.
///
/// This only works with path that have a single segment, e.g. `Option<T>`.
/// Paths with multiple segments, e.g. `std::option::Option<T>`, are not
/// supported and will be ignored.
pub fn extract_generic(ty: &syn::Type, name: &str) -> Option<syn::Type> {
    let check_name = |path: &syn::Path| {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.first().unwrap().ident == name
    };

    match ty {
        syn::Type::Path(path) if path.qself.is_none() && check_name(&path.path) => {
            let arguments = &path.path.segments.first().unwrap().arguments;
            // Should be one angle-bracketed param
            let arg = match arguments {
                PathArguments::AngleBracketed(params) if params.args.len() == 1 => {
                    params.args.first().unwrap()
                }
                _ => return None,
            };
            // The argument should be a type
            match arg {
                GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Parse description from #[doc] attributes.
///
/// Only the first attribute is parsed (corresponding to the first line of
/// documentation) https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
///
/// This function return error if the description is not found or if the
/// description is longer than 100 characters.
pub fn parse_doc(attrs: &[Attribute], span: Span) -> Result<String> {
    let Some(attr) = find_attr(attrs, "doc")  else {
        return Err(Error::new(
            span,
            "description is required (documentation comment or `desc` attribute)",
        ))
    };

    let meta = attr.meta.require_name_value()?;
    let Expr::Lit(expr) = &meta.value else {
        return Err(Error::new_spanned(&meta.value, "expected string literal"))
    };
    let Lit::Str(lit) = &expr.lit else {
        return Err(Error::new_spanned(&expr.lit, "expected string literal"))
    };

    let doc = lit.value().trim().to_string();

    match doc.chars().count() {
        1..=100 => Ok(doc),
        _ => Err(Error::new_spanned(
            lit,
            "description must be between 1 and 100 characters",
        )),
    }
}

/// Convert an [`Option<T>`] into a [`TokenStream`]
pub fn optional<T>(value: Option<T>) -> TokenStream
where
    T: ToTokens,
{
    match value {
        Some(value) => quote! { ::std::option::Option::Some(#value) },
        None => quote! {::std::option::Option::None },
    }
}

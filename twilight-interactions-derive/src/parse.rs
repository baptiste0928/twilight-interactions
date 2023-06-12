//! Utility functions to parse macro input.

use std::{collections::HashMap, fmt::Display, str::FromStr};

use proc_macro2::Span;
use syn::{
    meta::ParseNestedMeta, spanned::Spanned, Attribute, Error, Expr, ExprLit, Lit, Meta,
    MetaNameValue, Result,
};

/// Extracts type from an [`Option<T>`]
///
/// This function extracts the type in an [`Option<T>`]. It currently only works
/// with the `Option` syntax (not `std::option::Option` or similar).
pub fn extract_option(ty: &syn::Type) -> Option<syn::Type> {
    extract_type(ty, "Option")
}

pub fn extract_type(ty: &syn::Type, name: &str) -> Option<syn::Type> {
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
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => return None,
            };
            // The argument should be a type
            match arg {
                syn::GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Find an [`Attribute`] with a specific name
///
/// Returns the first match
pub fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    for attr in attrs {
        if let Some(ident) = attr.path().get_ident() {
            if *ident == name {
                return Some(attr);
            }
        }
    }

    None
}

/// Parse description from #[doc] attributes.
///
/// Only the first attribute is parsed (corresponding to the first line of
/// documentation) https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
pub fn parse_doc(attrs: &[Attribute], span: Span) -> Result<String> {
    let attr = match find_attr(attrs, "doc") {
        Some(attr) => attr,
        None => {
            return Err(Error::new(
                span,
                "description is required (documentation comment or `desc` attribute)",
            ))
        }
    };

    let value = match &attr.meta {
        Meta::NameValue(MetaNameValue { value, .. }) => value,
        _ => {
            return Err(Error::new(
                attr.span(),
                "failed to parse documentation attribute",
            ))
        }
    };

    let doc = match value {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) => lit.value().trim().to_string(),
        _ => {
            return Err(Error::new(
                attr.span(),
                "failed to parse documentation attribute",
            ))
        }
    };

    match doc.chars().count() {
        1..=100 => Ok(doc),
        _ => Err(Error::new(
            span,
            "description must be between 1 and 100 characters",
        )),
    }
}

/// Parsed list of named attributes like `#[command(rename = "name")]`.
///
/// Attributes are stored as a HashMap with String keys for fast lookups.
pub struct NamedAttrs<'a> {
    values: HashMap<String, AttrValue>,
    valid: &'a [&'a str],
}

impl<'a> NamedAttrs<'a> {
    /// Initialize a new [`NamedAttrs`] parser.
    ///
    /// A list of valid attribute names must be provided.
    pub fn new(valid: &'a [&'a str]) -> Self {
        Self {
            values: HashMap::new(),
            valid,
        }
    }

    /// Parse an [`Attribute`] into [`NamedAttrs`]
    ///
    /// This method should be used as an argument of [`Attribute::parse_nested_meta`]
    pub fn parse(&mut self, meta: ParseNestedMeta<'_>) -> Result<()> {
        let expected = || self.valid.join(", ");

        // Get name of the parameter as a String.
        let key = match meta.path.get_ident() {
            Some(ident) => ident.to_string(),
            None => {
                return Err(meta.error(format!("invalid parameter name (expected {}", expected())))
            }
        };

        // Ensure the parsed parameter is valid
        if !self.valid.contains(&&*key) {
            return Err(meta.error(format!("invalid parameter name (expected {})", expected())));
        }

        self.values.insert(key, AttrValue(meta.value()?.parse()?));

        Ok(())
    }

    /// Get a parsed parameter by name
    pub fn get(&self, name: &str) -> Option<&AttrValue> {
        self.values.get(name)
    }
}

/// Parsed attribute value.
///
/// Wrapper around a [`MetaNameValue`] reference with utility methods.
pub struct AttrValue(Lit);

impl AttrValue {
    /// Borrow the inner value
    pub fn inner(&self) -> &Lit {
        &self.0
    }

    pub fn parse_string(&self) -> Result<String> {
        match self.inner() {
            Lit::Str(inner) => Ok(inner.value().trim().to_string()),
            _ => Err(Error::new(
                self.0.span(),
                "invalid attribute type, expected string",
            )),
        }
    }

    pub fn parse_bool(&self) -> Result<bool> {
        match self.inner() {
            Lit::Bool(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "invalid attribute type, expected boolean",
            )),
        }
    }

    pub fn parse_int<N>(&self) -> Result<N>
    where
        N: FromStr,
        N::Err: Display,
    {
        match self.inner() {
            Lit::Int(inner) => inner.base10_parse(),
            _ => Err(Error::new(
                self.0.span(),
                "invalid attribute type, expected integer",
            )),
        }
    }
}

/// Parse function or item path.
pub fn parse_path(val: &AttrValue) -> Result<syn::Path> {
    let val = val.parse_string()?;

    syn::parse_str(&val)
}

/// Parse command or option name
pub fn parse_name(val: &AttrValue) -> Result<String> {
    let span = val.inner().span();
    let val = val.parse_string()?;

    // Command or option names must meet the following requirements:
    // - Length between 1 and 32 characters
    // - Only alphanumeric character allowed (except '-' and '_')
    // - Must be lowercase when possible
    //
    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming

    match val.chars().count() {
        1..=32 => (),
        _ => return Err(Error::new(span, "name must be between 1 and 32 characters")),
    }

    for char in val.chars() {
        if !char.is_alphanumeric() && char != '-' && char != '_' {
            return Err(Error::new(
                span,
                format!("name must only contain word characters, found invalid character `{char}`"),
            ));
        }

        if char.to_lowercase().to_string() != char.to_string() {
            return Err(Error::new(
                span,
                format!("name must be in lowercase, found invalid character `{char}`"),
            ));
        }
    }

    Ok(val)
}

/// Parse command or option description
pub fn parse_desc(val: &AttrValue) -> Result<String> {
    let span = val.inner().span();
    let val = val.parse_string()?;

    match val.chars().count() {
        1..=100 => Ok(val),
        _ => Err(Error::new(
            span,
            "description must be between 1 and 100 characters",
        )),
    }
}

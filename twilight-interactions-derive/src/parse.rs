//! Utility functions to parse macro input.

use std::collections::HashMap;

use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Error, Lit, Meta, MetaNameValue, Result};

/// Extracts type from an [`Option<T>`]
///
/// This function extracts the type in an [`Option<T>`]. It currently only works
/// with the `Option` syntax (not the `std::option::Option` or similar).
pub fn extract_option(ty: &syn::Type) -> Option<syn::Type> {
    fn check_name(path: &syn::Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.first().unwrap().ident == "Option"
    }

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
        if let Some(ident) = attr.path.get_ident() {
            if *ident == name {
                return Some(attr);
            }
        }
    }

    None
}

/// Parse description from #[doc] attributes.
///
/// Only fist attribute is parsed (corresponding to the first line of documentation)
/// https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
pub fn parse_doc(attrs: &[Attribute], span: Span) -> Result<String> {
    let attr = match find_attr(attrs, "doc") {
        Some(attr) => attr,
        None => {
            return Err(Error::new(
                span,
                "Description is required (documentation comment or `desc` attribute)",
            ))
        }
    };

    let doc = match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(inner),
            ..
        }) => inner.value().trim().to_owned(),
        _ => {
            return Err(Error::new(
                attr.span(),
                "Failed to parse documentation attribute",
            ))
        }
    };

    match doc.chars().count() {
        1..=100 => Ok(doc),
        _ => Err(Error::new(
            span,
            "Description must be between 1 and 100 characters",
        )),
    }
}

/// Parsed list of named attributes like `#[command(rename = "name")]`.
///
/// Attributes are stored as a HashMap with String keys for fast lookups.
pub struct NamedAttrs(HashMap<String, AttrValue>);

impl NamedAttrs {
    /// Parse a [`Meta`] into [`NamedAttrs`]
    ///
    /// A list of expected parameters must be provided.
    pub fn parse(meta: Meta, expected: &[&str]) -> Result<Self> {
        // Ensure there is a list of parameters like `#[command(...)]`
        let list = match meta {
            Meta::List(list) => list,
            _ => return Err(Error::new(meta.span(), "Expected named parameters list")),
        };

        let expected = expected.join(", ");
        let mut values = HashMap::new();

        // Parse each item in parameters list
        for nested in list.nested {
            // Ensure each attribute is a name-value attribute like `rename = "name"`
            let inner = match nested {
                syn::NestedMeta::Meta(Meta::NameValue(item)) => item,
                _ => return Err(Error::new(nested.span(), "Expected named parameter")),
            };

            // Extract name of each attribute as String. It must be a single segment path.
            let key = match inner.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => {
                    return Err(Error::new(
                        inner.path.span(),
                        format!("Invalid parameter name (expected {})", expected),
                    ))
                }
            };

            // Ensure the parsed parameter is expected
            if !expected.contains(&&*key) {
                return Err(Error::new(
                    inner.path.span(),
                    format!("Invalid parameter name (expected {})", expected),
                ));
            }

            values.insert(key, AttrValue(inner.lit));
        }

        Ok(Self(values))
    }

    /// Get a parsed parameter by name
    pub fn get(&self, name: &str) -> Option<&AttrValue> {
        self.0.get(name)
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
            Lit::Str(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Invalid attribute type, expected string",
            )),
        }
    }

    pub fn parse_bool(&self) -> Result<bool> {
        match self.inner() {
            Lit::Bool(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Invalid attribute type, expected boolean",
            )),
        }
    }
}

impl Spanned for AttrValue {
    fn span(&self) -> Span {
        self.0.span()
    }
}

/// Parse command or option name
pub fn parse_name(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    // Command or option names must meet the following requirements:
    // - Length between 1 and 32 characters
    // - Only alphanumeric character allowed (except '-' and '_')
    // - Must be lowercase when possible
    //
    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming

    match val.chars().count() {
        1..=32 => (),
        _ => return Err(Error::new(span, "Name must be between 1 and 32 characters")),
    }

    for char in val.chars() {
        if !char.is_alphanumeric() && char != '-' && char != '_' {
            return Err(Error::new(
                span,
                format!(
                    "Name must only contain word characters, found invalid character `{}`",
                    char
                ),
            ));
        }

        if char.to_lowercase().to_string() != char.to_string() {
            return Err(Error::new(
                span,
                format!(
                    "Name must be in lowercase, found invalid character `{}`",
                    char
                ),
            ));
        }
    }

    Ok(val)
}

/// Parse command or option description
pub fn parse_desc(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    match val.chars().count() {
        1..=100 => Ok(val),
        _ => Err(Error::new(
            span,
            "Description must be between 1 and 100 characters",
        )),
    }
}

use std::collections::HashMap;

use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Error, Lit, Meta, Result};

/// Find an [`Attribute`] with a specific name
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

/// Parsed field attribute
#[derive(Default)]
pub(crate) struct FieldAttributes {
    /// Rename the field to the given name
    pub(crate) rename: Option<String>,
    /// Overwrite the field description
    pub(crate) desc: Option<String>,
    /// Limit to specific channel types
    pub(crate) channel_types: Vec<()>,
}

impl FieldAttributes {
    /// Parse a single [`Attribute`]
    pub(crate) fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["rename", "desc", "channel_types"])?;

        let rename = attrs.get("rename").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_description).transpose()?;

        Ok(Self {
            rename,
            desc,
            channel_types: Vec::new(),
        })
    }
}

/// Parse command or option name.
fn parse_name(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    match val.chars().count() {
        1..=32 => Ok(val),
        _ => Err(Error::new(span, "Name must be between 1 and 32 characters")),
    }
}

/// Parse command or option description
fn parse_description(val: &AttrValue) -> Result<String> {
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

/// Parsed list of named attributes like `#[command(rename = "name")]`.
///
/// Attributes are stored as a HashMap with String keys for fast lookups.
struct NamedAttrs(HashMap<String, AttrValue>);

impl NamedAttrs {
    /// Parse a [`Meta`] into [`NamedAttrs`]
    ///
    /// A list of expected parameters must be provided.
    fn parse(meta: Meta, expected: &[&str]) -> Result<Self> {
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
    fn get(&self, name: &str) -> Option<&AttrValue> {
        self.0.get(name)
    }
}

/// Parsed attribute value.
///
/// Wrapper around a [`MetaNameValue`] reference with utility methods.
struct AttrValue(Lit);

impl AttrValue {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn parse_string(&self) -> Result<String> {
        match &self.0 {
            Lit::Str(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Invalid attribute type, expected string",
            )),
        }
    }
}

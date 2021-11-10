//! Parsing of option attribute

use syn::{spanned::Spanned, Attribute, Error, Result};

use crate::parse::{AttrValue, NamedAttrs};

/// Parsed variant attribute
pub struct VariantAttribute {
    /// Name of the choice (shown to users)
    pub name: String,
    /// Value of the choice
    pub value: ChoiceValue,
}

impl VariantAttribute {
    /// Parse a single [`Attribute`].
    ///
    /// If no [`ChoiceKind`] is provided, the type is inferred from value.
    pub fn parse(attr: &Attribute, kind: Option<ChoiceKind>) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["name", "value"])?;

        let name = match attrs.get("name") {
            Some(val) => parse_name(val)?,
            None => return Err(Error::new(attr.span(), "Missing required attribute `name`")),
        };

        let value = match attrs.get("value") {
            Some(val) => ChoiceValue::parse(val, kind)?,
            None => {
                return Err(Error::new(
                    attr.span(),
                    "Missing required attribute `value`",
                ))
            }
        };

        Ok(Self { name, value })
    }
}

/// Value of a parsed choice
#[derive(Debug, Clone)]
pub enum ChoiceValue {
    String(String),
    Int(i64),
    Number(f64),
}

impl ChoiceValue {
    /// Parse a [`AttrValue`] into a [`ChoiceValue`]
    pub fn parse(val: &AttrValue, kind: Option<ChoiceKind>) -> Result<Self> {
        let parsed = match val.inner() {
            syn::Lit::Str(inner) => Self::String(inner.value()),
            syn::Lit::Int(inner) => Self::Int(inner.base10_parse()?),
            syn::Lit::Float(inner) => Self::Number(inner.base10_parse()?),
            _ => {
                return Err(Error::new(
                    val.span(),
                    "Invalid attribute type, expected string, integer or float",
                ))
            }
        };

        // Ensure parsed type is expected type
        if let Some(kind) = kind {
            if parsed.kind() != kind {
                return Err(Error::new(
                    val.span(),
                    format!("Invalid attribute type, expected {}", kind.name()),
                ));
            }
        }

        Ok(parsed)
    }

    /// Get the [`ChoiceKind`] corresponding to this value
    pub fn kind(&self) -> ChoiceKind {
        match self {
            ChoiceValue::String(_) => ChoiceKind::String,
            ChoiceValue::Int(_) => ChoiceKind::Integer,
            ChoiceValue::Number(_) => ChoiceKind::Number,
        }
    }
}

/// Type of parsed variants (inferred from value)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChoiceKind {
    String,
    Integer,
    Number,
}

impl ChoiceKind {
    /// Get the type name
    fn name(&self) -> &'static str {
        match self {
            ChoiceKind::String => "string",
            ChoiceKind::Integer => "integer",
            ChoiceKind::Number => "float",
        }
    }
}

/// Parse choice name
fn parse_name(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
    match val.chars().count() {
        1..=100 => Ok(val),
        _ => Err(Error::new(
            span,
            "Name must be between 1 and 100 characters",
        )),
    }
}

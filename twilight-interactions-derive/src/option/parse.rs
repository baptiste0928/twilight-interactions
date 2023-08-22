use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Error, Fields, Lit, Result, Variant};

use crate::parse::{
    attribute::{NamedAttrs, ParseAttribute, ParseSpanned},
    parsers::{ChoiceName, FunctionPath},
    syntax::find_attr,
};

/// Parsed enum variants.
pub struct ParsedVariant {
    pub span: Span,
    pub ident: Ident,
    pub attribute: VariantAttribute,
    pub kind: ChoiceKind,
}

impl ParsedVariant {
    /// Parse an iterator of syn [`Variant`].
    ///
    /// The inferred [`OptionKind`] is also returned.
    pub fn from_variants(
        variants: impl IntoIterator<Item = Variant>,
        input_span: Span,
    ) -> Result<(Vec<Self>, ChoiceKind)> {
        let mut iter = variants.into_iter();

        // Parse the fist variant to infer the type
        let first = match iter.next() {
            Some(variant) => Self::from_variant(variant, None)?,
            None => {
                return Err(Error::new(
                    input_span,
                    "enum must have at least one variant",
                ))
            }
        };
        let choice_kind = first.kind;

        // Parse other variants
        let mut variants = vec![first];
        for variant in iter {
            variants.push(Self::from_variant(variant, Some(choice_kind))?);
        }

        Ok((variants, choice_kind))
    }

    /// Parse a single syn [`Variant`].
    ///
    /// If no [`ChoiceKind`] is provided, the type is inferred from value.
    fn from_variant(variant: Variant, kind: Option<ChoiceKind>) -> Result<Self> {
        match variant.fields {
            Fields::Unit => (),
            _ => return Err(Error::new(variant.span(), "variant must be a unit variant")),
        }

        let attribute = match find_attr(&variant.attrs, "option") {
            Some(attr) => VariantAttribute::parse(attr, kind)?,
            None => {
                return Err(Error::new(
                    variant.span(),
                    "missing required #[option(...)] attribute",
                ))
            }
        };

        Ok(Self {
            span: variant.span(),
            ident: variant.ident,
            kind: attribute.value.kind(),
            attribute,
        })
    }
}

/// Parsed variant attribute
pub struct VariantAttribute {
    /// Name of the choice (shown to users)
    pub name: ChoiceName,
    /// Localizations dictionary for the choice name
    pub name_localizations: Option<FunctionPath>,
    /// Value of the choice
    pub value: ChoiceValue,
}

impl VariantAttribute {
    /// Parse a single [`Attribute`].
    ///
    /// If no [`ChoiceKind`] is provided, the type is inferred from value.
    pub fn parse(attr: &Attribute, kind: Option<ChoiceKind>) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, &["name", "name_localizations", "value"])?;

        // Ensure the parsed type is the same as the inferred one
        let value: ParseSpanned<ChoiceValue> = parser.required("value")?;
        if let Some(kind) = kind {
            if value.inner.kind() != kind {
                return Err(Error::new(
                    value.span,
                    format!("invalid attribute type, expected {}", kind.name()),
                ));
            }
        }

        Ok(Self {
            name: parser.required("name")?,
            name_localizations: parser.optional("name_localizations")?,
            value: value.inner,
        })
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
    /// Get the [`ChoiceKind`] corresponding to this value
    pub fn kind(&self) -> ChoiceKind {
        match self {
            ChoiceValue::String(_) => ChoiceKind::String,
            ChoiceValue::Int(_) => ChoiceKind::Integer,
            ChoiceValue::Number(_) => ChoiceKind::Number,
        }
    }
}

impl ParseAttribute for ChoiceValue {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let parsed = match input {
            Lit::Str(inner) => Self::String(inner.value()),
            Lit::Int(inner) => Self::Int(inner.base10_parse()?),
            Lit::Float(inner) => Self::Number(inner.base10_parse()?),
            _ => {
                return Err(Error::new_spanned(
                    input,
                    "expected string, integer or float point literal",
                ))
            }
        };

        Ok(parsed)
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

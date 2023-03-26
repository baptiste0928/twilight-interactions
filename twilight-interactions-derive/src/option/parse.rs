use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Error, Fields, Result, Variant};

use crate::parse::{find_attr, parse_path, AttrValue, NamedAttrs};

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
    pub name: String,
    /// Localizations dictionary for the choice name
    pub name_localizations: Option<syn::Path>,
    /// Value of the choice
    pub value: ChoiceValue,
}

impl VariantAttribute {
    /// Parse a single [`Attribute`].
    ///
    /// If no [`ChoiceKind`] is provided, the type is inferred from value.
    pub fn parse(attr: &Attribute, kind: Option<ChoiceKind>) -> Result<Self> {
        let mut parser = NamedAttrs::new(&["name", "name_localizations", "value"]);

        attr.parse_nested_meta(|meta| parser.parse(meta))?;

        let name = match parser.get("name") {
            Some(val) => parse_name(val)?,
            None => return Err(Error::new(attr.span(), "missing required attribute `name`")),
        };
        let name_localizations = parser
            .get("name_localizations")
            .map(parse_path)
            .transpose()?;
        let value = match parser.get("value") {
            Some(val) => ChoiceValue::parse(val, kind)?,
            None => {
                return Err(Error::new(
                    attr.span(),
                    "missing required attribute `value`",
                ))
            }
        };

        Ok(Self {
            name,
            name_localizations,
            value,
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
    /// Parse a [`AttrValue`] into a [`ChoiceValue`]
    pub fn parse(val: &AttrValue, kind: Option<ChoiceKind>) -> Result<Self> {
        let parsed = match val.inner() {
            syn::Lit::Str(inner) => Self::String(inner.value()),
            syn::Lit::Int(inner) => Self::Int(inner.base10_parse()?),
            syn::Lit::Float(inner) => Self::Number(inner.base10_parse()?),
            _ => {
                return Err(Error::new(
                    val.inner().span(),
                    "invalid attribute type, expected string, integer or float",
                ))
            }
        };

        // Ensure parsed type is expected type
        if let Some(kind) = kind {
            if parsed.kind() != kind {
                return Err(Error::new(
                    val.inner().span(),
                    format!("invalid attribute type, expected {}", kind.name()),
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
    let span = val.inner().span();
    let val = val.parse_string()?;

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
    match val.chars().count() {
        1..=100 => Ok(val),
        _ => Err(Error::new(
            span,
            "name must be between 1 and 100 characters",
        )),
    }
}

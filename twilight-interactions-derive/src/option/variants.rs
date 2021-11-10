use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Error, Result, Variant};

use crate::parse::find_attr;

use super::attributes::{ChoiceKind, VariantAttribute};

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
                    "Enum must have at least one variant",
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
        let attribute = match find_attr(&variant.attrs, "option") {
            Some(attr) => VariantAttribute::parse(attr, kind)?,
            None => {
                return Err(Error::new(
                    variant.span(),
                    "Missing required #[option(...)] attribute",
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

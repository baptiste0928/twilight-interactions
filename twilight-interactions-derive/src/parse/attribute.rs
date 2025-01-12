//! Attribute parsing functions.
//!
//! This module exposes functions used to parse attributes in this crate.
//! - [`NamedAttrs`] parses a list of named attributes and allow to parse each
//!   of them using a custom parser function.
//! - [`ParseAttribute`] is used to parse a single attribute into a concrete
//!   type.

use std::fmt::Display;

use proc_macro2::{Ident, Span};
use syn::{meta::ParseNestedMeta, spanned::Spanned, Attribute, Error, Lit, Result};

/// Parse a list of named attributes like `#[command(rename = "name")]`.
///
/// This only support `(ident) = (literal)` syntax for simplicity. Collected
/// values can be parsed using the `optional` and `required` methods.
pub struct NamedAttrs {
    attr_span: Span,
    values: Vec<(Ident, Lit)>,
}

impl NamedAttrs {
    /// Initialize a new [`NamedAttrs`] parser and parse the provided [`Attribute`].
    ///
    /// A list of valid attribute arguments must be provided.
    pub fn parse(attr: &Attribute, valid: &[&str]) -> Result<Self> {
        let mut parser = Self {
            attr_span: attr.span(),
            values: Vec::new(),
        };

        attr.parse_nested_meta(|meta| parser.parse_meta(meta, valid))?;

        Ok(parser)
    }

    fn parse_meta(&mut self, meta: ParseNestedMeta, valid: &[&str]) -> Result<()> {
        let is_valid = |ident| valid.iter().any(|name| ident == name);

        let Some(ident) = meta.path.get_ident().filter(|i| is_valid(*i)) else {
            let expected = valid.join(", ");
            return Err(Error::new_spanned(
                meta.path,
                format!("invalid argument name (expected one of {expected})"),
            ));
        };

        let lit: Lit = meta.value()?.parse()?;
        self.values.push((ident.clone(), lit));

        Ok(())
    }

    /// Parse an optional attribute using the specified parser function.
    pub fn optional<T: ParseAttribute>(&mut self, name: &str) -> Result<Option<T>> {
        let Some(index) = self.values.iter().position(|(ident, _)| ident == name) else {
            return Ok(None);
        };

        let (_, lit) = self.values.remove(index);
        let parsed = T::parse_attribute(lit)?;

        Ok(Some(parsed))
    }

    /// Parse a required attribute using the specified parser function.
    ///
    /// If the attribute is not found, an error is returned.
    pub fn required<T: ParseAttribute>(&mut self, name: &str) -> Result<T> {
        let Some(parsed) = self.optional::<T>(name)? else {
            return Err(Error::new(
                self.attr_span,
                format!("missing required `{name}` argument"),
            ));
        };

        Ok(parsed)
    }
}

/// Parse an attribute literal into a concrete type.
pub trait ParseAttribute: Sized {
    fn parse_attribute(input: Lit) -> Result<Self>;
}

impl ParseAttribute for String {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let Lit::Str(lit) = input else {
            return Err(Error::new_spanned(input, "expected string literal"));
        };

        Ok(lit.value())
    }
}

impl ParseAttribute for bool {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let Lit::Bool(lit) = input else {
            return Err(Error::new_spanned(input, "expected boolean literal"));
        };

        Ok(lit.value)
    }
}

impl ParseAttribute for u16 {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let Lit::Int(lit) = input else {
            return Err(Error::new_spanned(input, "expected integer literal"));
        };

        lit.base10_parse()
    }
}

/// Capture the [`Span`] of a parsed attribute.
pub struct ParseSpanned<T> {
    pub span: Span,
    pub inner: T,
}

impl<T> ParseSpanned<T> {
    pub fn error(&self, message: impl Display) -> Error {
        Error::new(self.span, message)
    }
}

impl<T: ParseAttribute> ParseAttribute for ParseSpanned<T> {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let span = input.span();
        let inner = T::parse_attribute(input)?;

        Ok(Self { span, inner })
    }
}

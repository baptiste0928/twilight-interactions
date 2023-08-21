//! Attribute parsing functions.
//!
//! This module exposes functions used to parse attributes in this crate.
//! - [`NamedAttrs`] parses a list of named attributes and allow to parse each
//!   of them using a custom parser function.
//! - [`ParseAttribute`] is used to parse a single attribute into a concrete
//!  type.

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    meta::ParseNestedMeta,
    parse::{ParseStream, Parser},
    spanned::Spanned,
    Attribute, Error, Expr, LitBool, LitInt, LitStr, Path, Result,
};

/// Parse a list of named attributes like `#[command(rename = "name")]`.
///
/// This type collects all attributes matching allowed names as a list of
/// [`TokenStream`]s, and provides methods to parse values from them.
pub struct NamedAttrs {
    attr_span: Span,
    values: Vec<(Path, TokenStream)>,
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
        let is_valid = valid.iter().any(|n| meta.path.is_ident(n));

        if !is_valid {
            let expected = valid.join(", ");
            return Err(meta.error(format!(
                "invalid argument name (expected one of {expected})"
            )));
        };

        let expr: Expr = meta.value()?.parse()?;

        self.values
            .push((meta.path.clone(), expr.into_token_stream()));

        Ok(())
    }

    /// Parse an optional attribute using the specified parser function.
    pub fn optional<T: ParseAttribute>(&mut self, name: &str) -> Result<Option<T>> {
        let index = match self.values.iter().position(|(n, _)| n.is_ident(name)) {
            Some(index) => index,
            None => return Ok(None),
        };

        let (_, tokens) = self.values.remove(index);
        let parsed = T::parse_attribute.parse2(tokens)?;

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
            ))
        };

        Ok(parsed)
    }
}

/// Parse an attribute into a concrete type.
///
/// This trait is identical to the [`syn::parse::Parse`] trait with a different
/// method name.
pub trait ParseAttribute: Sized {
    fn parse_attribute(input: ParseStream) -> Result<Self>;
}

impl ParseAttribute for String {
    fn parse_attribute(input: ParseStream) -> Result<Self> {
        let lit: LitStr = input.parse()?;

        Ok(lit.value())
    }
}

impl ParseAttribute for bool {
    fn parse_attribute(input: ParseStream) -> Result<Self> {
        let lit: LitBool = input.parse()?;

        Ok(lit.value)
    }
}

impl ParseAttribute for u16 {
    fn parse_attribute(input: ParseStream) -> Result<Self> {
        let lit: LitInt = input.parse()?;

        lit.base10_parse()
    }
}

/// Capture the [`Span`] of a parsed attribute.
pub struct ParsedSpanned<T> {
    pub span: Span,
    pub inner: T,
}

impl<T: ParseAttribute> ParseAttribute for ParsedSpanned<T> {
    fn parse_attribute(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let inner = T::parse_attribute(input)?;

        Ok(Self { span, inner })
    }
}

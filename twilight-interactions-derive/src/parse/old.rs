//! Utility functions to parse macro input.

use std::{fmt::Display, str::FromStr};

use proc_macro2::{Span, TokenTree, TokenStream};
use syn::{
    meta::ParseNestedMeta,
    parse::{ParseBuffer, ParseStream, Parser},
    spanned::Spanned,
    Attribute, Error, Expr, ExprLit, Lit, LitBool, LitFloat, LitInt, LitStr, Meta, MetaNameValue,
    Path, Result,
};

/// Parse an integer.
pub fn parse_int<N>(input: ParseStream<'_>) -> Result<N>
where
    N: FromStr,
    N::Err: Display,
{
    let lit: LitInt = input.parse()?;

    lit.base10_parse()
}

/// Parse a float.
pub fn parse_float<N>(input: ParseStream<'_>) -> Result<N>
where
    N: FromStr,
    N::Err: Display,
{
    let lit: LitFloat = input.parse()?;

    lit.base10_parse()
}

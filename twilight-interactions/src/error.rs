//! Error types used by the crate.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};


/// Error when parsing a command option.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The name of the option that caused the error.
    pub option: String,
    /// The type of the error.
    pub kind: ParseErrorType,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "failed to parse option `{}`: ", self.option)?;

        match &self.kind {
            ParseErrorType::InvalidType(name) => write!(f, "invalid type, found {}", name),
            ParseErrorType::ParseId(id) => write!(f, "`{}` is not a valid discord id", id),
            ParseErrorType::LookupFailed(id) => write!(f, "failed to resolve `{}`", id),
        }
    }
}

/// Type of [`ParseError`] that occured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType {
    /// Found an invalid option type.
    InvalidType(&'static str),
    /// Failed to parse a Discord ID.
    ParseId(String),
    /// Failed to resolve data associated with an ID.
    LookupFailed(u64),
}

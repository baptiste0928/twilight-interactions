//! Error types used by the crate.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Error when parsing a command option.
///
/// This error type is returned by the [`CommandModel::from_interaction`] method.
///
/// [`CommandModel::from_interaction`]: crate::CommandModel::from_interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// The name of the option field that caused the error.
    pub field: String,
    /// The type of the error.
    pub kind: ParseErrorType,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "failed to parse option `{}`: ", self.field)?;

        match &self.kind {
            ParseErrorType::InvalidType(name) => write!(f, "invalid type, found {}", name),
            ParseErrorType::ParseId(id) => write!(f, "`{}` is not a valid discord id", id),
            ParseErrorType::LookupFailed(id) => write!(f, "failed to resolve `{}`", id),
            ParseErrorType::UnknownField => write!(f, "unknown field"),
            ParseErrorType::RequiredField => write!(f, "missing required field"),
        }
    }
}

/// Type of [`ParseError`] that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType {
    /// Found an invalid option type.
    InvalidType(&'static str),
    /// Failed to parse a Discord ID.
    ParseId(String),
    /// Failed to resolve data associated with an ID.
    LookupFailed(u64),
    /// Missing a required option field
    RequiredField,
    /// Received an unknown option field
    UnknownField,
}

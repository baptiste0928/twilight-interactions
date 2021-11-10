//! Error types used by the crate.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU64,
};

use twilight_model::application::command::CommandOptionType;

/// Error when parsing a command option.
///
/// This error type is returned by the [`CommandModel::from_interaction`] method.
///
/// [`CommandModel::from_interaction`]: crate::command::CommandModel::from_interaction
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
            ParseErrorType::InvalidType(ty) => write!(f, "invalid type, found {}", ty.kind()),
            ParseErrorType::InvalidChoice(choice) => {
                write!(f, "invalid choice value, found `{}`", choice)
            }
            ParseErrorType::LookupFailed(id) => write!(f, "failed to resolve `{}`", id),
            ParseErrorType::UnknownField => write!(f, "unknown field"),
            ParseErrorType::RequiredField => write!(f, "missing required field"),
        }
    }
}

/// Type of [`ParseError`] that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType {
    /// Received an invalid option type.
    InvalidType(CommandOptionType),
    /// Received an invalid value on choice option type.
    InvalidChoice(String),
    /// Failed to resolve data associated with an ID.
    LookupFailed(NonZeroU64),
    /// Missing a required option field.
    RequiredField,
    /// Received an unknown option field.
    UnknownField,
}

//! Error types used by the crate.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use twilight_model::{
    application::command::{CommandOptionType, Number},
    channel::ChannelType,
};

/// Error when parsing a command.
///
/// This error type is returned by the [`CommandModel::from_interaction`]
/// method.
///
/// [`CommandModel::from_interaction`]: crate::command::CommandModel::from_interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Received empty option list.
    ///
    /// This error is only returned when parsing subcommands.
    EmptyOptions,
    /// Error when parsing a command option.
    Option(ParseOptionError),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseError::EmptyOptions => write!(f, "received an empty option list"),
            ParseError::Option(error) => error.fmt(f),
        }
    }
}

/// Error when parsing a command option.
///
/// This type is used by [`ParseError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseOptionError {
    /// The name of the option field that caused the error.
    pub field: String,
    /// The type of the error.
    pub kind: ParseOptionErrorType,
}

impl Error for ParseOptionError {}

impl Display for ParseOptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "failed to parse option `{}`: ", self.field)?;

        match &self.kind {
            ParseOptionErrorType::InvalidType(ty) => write!(f, "invalid type, found {}", ty.kind()),
            ParseOptionErrorType::InvalidChoice(choice) => {
                write!(f, "invalid choice value, found `{}`", choice)
            }
            ParseOptionErrorType::IntegerOutOfRange(val) => {
                write!(f, "out of range integer, received `{}`", val)
            }
            ParseOptionErrorType::NumberOutOfRange(val) => {
                write!(f, "out of range number, received `{}`", val.0)
            }
            ParseOptionErrorType::InvalidChannelType(kind) => {
                write!(f, "invalid channel type, received `{}`", kind.name())
            }
            ParseOptionErrorType::LookupFailed(id) => write!(f, "failed to resolve `{}`", id),
            ParseOptionErrorType::UnknownField => write!(f, "unknown field"),
            ParseOptionErrorType::UnknownSubcommand => write!(f, "unknown subcommand"),
            ParseOptionErrorType::RequiredField => write!(f, "missing required field"),
        }
    }
}

/// Type of [`ParseOptionError`] that occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseOptionErrorType {
    /// Received an invalid option type.
    InvalidType(CommandOptionType),
    /// Received an invalid value on choice option type.
    InvalidChoice(String),
    /// Received an out of range integer.
    IntegerOutOfRange(i64),
    /// Received an out of range floating point number.
    NumberOutOfRange(Number),
    /// Received an invalid channel type.
    InvalidChannelType(ChannelType),
    /// Failed to resolve data associated with an ID.
    LookupFailed(u64),
    /// Missing a required option field.
    RequiredField,
    /// Received an unknown option field.
    UnknownField,
    /// Received an unknown subcommand.
    UnknownSubcommand,
}

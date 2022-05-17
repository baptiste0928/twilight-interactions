//! Implementation of `CommandModel` and `CreateCommand` macros for enums
//! (subcommands).

mod command_model;
mod create_command;
mod parse;

pub use command_model::impl_command_model;
pub use create_command::impl_create_command;

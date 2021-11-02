//! Implementation of `CommandModel` and `CreateCommand` derive macros.

mod attributes;
mod fields;

mod command_model;
mod create_command;

pub use command_model::impl_command_model;
pub use create_command::impl_create_command;

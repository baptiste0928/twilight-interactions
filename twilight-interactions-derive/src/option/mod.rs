//! Implementation of `CommandOption` and `CreateOption` derive macros.

mod command_option;
mod create_option;
mod parse;

pub use command_option::{dummy_command_option, impl_command_option};
pub use create_option::{dummy_create_option, impl_create_option};

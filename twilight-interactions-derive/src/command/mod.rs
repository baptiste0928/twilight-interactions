//! Implementation of `CommandModel` and `CreateCommand` derive macros.

mod impls;

mod description;
mod model;
mod subcommand;

pub use impls::{
    dummy_command_model, dummy_create_command, impl_command_model, impl_create_command,
};

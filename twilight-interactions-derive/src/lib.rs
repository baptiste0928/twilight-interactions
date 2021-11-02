//! # twilight-interactions-derive
//!
//! This crate provide derive macros for the `twilight-interactions` crate.
//!
//! Please refer to the `twilight-interactions` documentation for further information.

mod command;
mod parse;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the the `CommandModel` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CommandModel, attributes(command))]
pub fn command_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match command::impl_command_model(input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Derive macro for the the `CreateCommand` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CreateCommand, attributes(command))]
pub fn create_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match command::impl_create_command(input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

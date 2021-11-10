//! # twilight-interactions-derive
//!
//! This crate provide derive macros for the `twilight-interactions` crate.
//!
//! Please refer to the `twilight-interactions` documentation for further information.

mod command;
mod option;
mod parse;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the the `CommandModel` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CommandModel, attributes(command))]
pub fn command_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    match command::impl_command_model(input) {
        Ok(output) => output.into(),
        Err(error) => command::dummy_command_model(ident, error).into(),
    }
}

/// Derive macro for the the `CreateCommand` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CreateCommand, attributes(command))]
pub fn create_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    match command::impl_create_command(input) {
        Ok(output) => output.into(),
        Err(error) => command::dummy_create_command(ident, error).into(),
    }
}

/// Derive macro for the the `CommandOption` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CommandOption, attributes(option))]
pub fn command_option(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    match option::impl_command_option(input) {
        Ok(output) => output.into(),
        Err(error) => option::dummy_command_option(ident, error).into(),
    }
}

/// Derive macro for the the `CreateOption` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CreateOption, attributes(option))]
pub fn create_option(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    match option::impl_create_option(input) {
        Ok(output) => output.into(),
        Err(error) => option::dummy_create_option(ident, error).into(),
    }
}

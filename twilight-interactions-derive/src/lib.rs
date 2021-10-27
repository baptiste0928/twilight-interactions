//! # twilight-interactions-derive
//!
//! This crate provide derive macros for the `twilight-interactions` crate.
//!
//! Please refer to the `twilight-interactions` documentation for further information.

mod attributes;
mod command_model;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the the `CommandModel` trait.
///
/// See the documentation of the trait for more information about usage of this macro.
#[proc_macro_derive(CommandModel, attributes(command))]
pub fn command_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match command_model::impl_command_model(input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Extracts type from an [`Option<T>`]
///
/// This function extracts the type in an [`Option<T>`]. It currently only works
/// with the `Option` syntax (not the `std::option::Option` or similar).
fn extract_option(ty: &syn::Type) -> Option<syn::Type> {
    fn check_name(path: &syn::Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.first().unwrap().ident == "Option"
    }

    match ty {
        syn::Type::Path(path) if path.qself.is_none() && check_name(&path.path) => {
            let arguments = &path.path.segments.first().unwrap().arguments;
            // Should be one angle-bracketed param
            let arg = match arguments {
                syn::PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => return None,
            };
            // The argument should be a type
            match arg {
                syn::GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

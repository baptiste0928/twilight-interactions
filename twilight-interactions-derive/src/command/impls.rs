use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Result};

/// Implementation of the `CommandModel` derive macro
pub fn impl_command_model(input: DeriveInput) -> Result<TokenStream> {
    let span = input.span();

    match input.data.clone() {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => super::model::impl_command_model(input, Some(fields)),
            Fields::Unit => super::model::impl_command_model(input, None),
            _ => Err(Error::new(
                span,
                "`CommandModel` can only be applied to structs with named fields or unit structs",
            )),
        },
        Data::Enum(data) => super::subcommand::impl_command_model(input, data.variants),
        _ => Err(Error::new(
            span,
            "`CommandModel` can only be applied to structs or enums",
        )),
    }
}

/// Dummy implementation of the `CommandModel` trait in case of macro error
pub fn dummy_command_model(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::command::CommandModel for #ident {
            fn from_interaction(
                data: ::twilight_interactions::command::CommandInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                ::std::unimplemented!()
            }
        }
    }
}

/// Implementation of the `CreateCommand` derive macro
pub fn impl_create_command(input: DeriveInput) -> Result<TokenStream> {
    let span = input.span();

    match input.data.clone() {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => super::model::impl_create_command(input, Some(fields)),
            Fields::Unit => super::model::impl_create_command(input, None),
            _ => Err(Error::new(
                span,
                "`CreateCommand` can only be applied to structs with named fields or unit structs",
            )),
        },
        Data::Enum(data) => super::subcommand::impl_create_command(input, data.variants),
        _ => Err(Error::new(
            span,
            "`CreateCommand` can only be applied to structs or enums",
        )),
    }
}

/// Dummy implementation of the `CreateCommand` trait in case of macro error
pub fn dummy_create_command(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::command::CreateCommand for #ident {
            const NAME: &'static str = "";

            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                ::std::unimplemented!()
            }
        }
    }
}

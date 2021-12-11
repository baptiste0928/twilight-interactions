use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result};

/// Implementation of the CommandModel derive macro
pub fn impl_command_model(input: DeriveInput) -> Result<TokenStream> {
    let span = input.span();

    match input.data.clone() {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => super::model::impl_command_model(input, fields),
        Data::Enum(DataEnum { variants, .. }) => {
            super::subcommand::impl_command_model(input, variants)
        }
        Data::Struct(_) => Err(Error::new(
            span,
            "`CommandModel` can only be applied to structs with named fields",
        )),
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
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => super::model::impl_create_command(input, fields),
        Data::Enum(DataEnum { variants, .. }) => {
            super::subcommand::impl_create_command(input, variants)
        }
        Data::Struct(_) => Err(Error::new(
            span,
            "`CreateCommand` can only be applied to structs with named fields",
        )),
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
            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                ::std::unimplemented!()
            }
        }
    }
}

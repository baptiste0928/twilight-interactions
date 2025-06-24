use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields};

pub fn impl_create_modal(input: DeriveInput) -> syn::Result<TokenStream> {
    let span = input.span();

    match input.data.clone() {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => super::create_modal::impl_create_modal(input, fields),
            _ => Err(Error::new(
                span,
                "`CreateModal` can only be applied to structs with named fields",
            )),
        },
        _ => Err(Error::new(
            span,
            "`CreateModal` can only be applied to structs",
        )),
    }
}

pub fn dummy_create_modal(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::modal::CreateModal for #ident {
            const CUSTOM_ID: &'static str = "";

            fn create_modal() -> ::twilight_interactions::modal::ModalData {
                ::std::unimplemented!()
            }
        }
    }
}

/// Implementation of the `ModalModel` derive macro
pub fn impl_modal_model(input: DeriveInput) -> syn::Result<TokenStream> {
    let span = input.span();

    match input.data.clone() {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => super::modal_model::impl_modal_model(input, Some(fields)),
            Fields::Unit => super::modal_model::impl_modal_model(input, None),
            _ => Err(Error::new(
                span,
                "`ModalModel` can only be applied to structs with named fields or unit structs",
            )),
        },
        _ => Err(Error::new(
            span,
            "`ModalModel` can only be applied to structs",
        )),
    }
}

/// Dummy implementation of the `ModalModel` trait in case of macro error
pub fn dummy_modal_model(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::modal::ModalModel for #ident {
            fn from_interaction(
                data: ::twilight_interactions::modal::ModalInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                ::std::unimplemented!()
            }
        }
    }
}

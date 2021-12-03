use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, DeriveInput, Error, Ident, Result};

use crate::option::{attributes::ChoiceValue, variants::ParsedVariant};

use super::attributes::ChoiceKind;

pub fn impl_create_option(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let input_span = input.span();

    let (variants, kind) = match input.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            ParsedVariant::from_variants(variants, input_span)?
        }
        _ => {
            return Err(Error::new(
                input_span,
                "`#[derive(CommandOption)] can only be applied to enums",
            ))
        }
    };

    let vec_capacity = variants.len();
    let choice_variants = variants.iter().map(choice_variant);
    let command_option = command_option(kind);

    Ok(quote! {
        impl ::twilight_interactions::command::CreateOption for #ident {
            fn create_option(
                data: ::twilight_interactions::command::internal::CommandOptionData,
            ) -> ::twilight_model::application::command::CommandOption {
                let mut choices = ::std::vec::Vec::with_capacity(#vec_capacity);

                #(#choice_variants)*

                #command_option
            }
        }
    })
}

pub fn dummy_create_option(ident: Ident, error: Error) -> TokenStream {
    let error = error.to_compile_error();

    quote! {
        #error

        impl ::twilight_interactions::command::CreateOption for #ident {
            fn create_option(
                data: ::twilight_interactions::command::internal::CommandOptionData,
            ) -> ::twilight_model::application::command::CommandOption {
                ::std::unimplemented!()
            }
        }
    }
}

/// Generate push instruction for a given variant
fn choice_variant(variant: &ParsedVariant) -> TokenStream {
    let name = &variant.attribute.name;
    let value = match &variant.attribute.value {
        ChoiceValue::String(val) => quote! { ::std::convert::From::from(#val) },
        ChoiceValue::Int(val) => val.to_token_stream(),
        ChoiceValue::Number(val) => quote! { twilight_model::application::command::Number(#val) },
    };
    let type_path = match variant.kind {
        ChoiceKind::String => quote! { String },
        ChoiceKind::Integer => quote! { Int },
        ChoiceKind::Number => quote! { Number },
    };

    quote! {
        choices.push(::twilight_model::application::command::CommandOptionChoice::#type_path {
            name: ::std::convert::From::from(#name),
            value: #value,
        });
    }
}

/// Generate command option
fn command_option(kind: ChoiceKind) -> TokenStream {
    let path = match kind {
        ChoiceKind::String => quote!(String(data.into_choice(choices))),
        ChoiceKind::Integer => quote!(Integer(data.into_number(choices))),
        ChoiceKind::Number => quote!(Number(data.into_number(choices))),
    };

    quote!(::twilight_model::application::command::CommandOption::#path)
}

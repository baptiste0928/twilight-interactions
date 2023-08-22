use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput, Result, Variant};

use super::parse::ParsedVariant;

/// Implementation of `CommandModel` derive macro
pub fn impl_command_model(
    input: DeriveInput,
    variants: impl IntoIterator<Item = Variant>,
) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let variants = ParsedVariant::from_variants(variants, input.span())?;

    let variants_match_arms = variants.iter().map(variant_match_arm);

    Ok(quote! {
        impl #generics ::twilight_interactions::command::CommandModel for #ident #generics #where_clause {
            fn from_interaction(
                __data: ::twilight_interactions::command::CommandInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                if __data.options.is_empty() {
                    return std::result::Result::Err(twilight_interactions::error::ParseError::EmptyOptions);
                }

                let mut __options = __data.options;
                let __opt = __options.swap_remove(0);

                match &*__opt.name {
                    #(#variants_match_arms,)*
                    __other => ::std::result::Result::Err(
                        ::twilight_interactions::error::ParseError::Option(
                            ::twilight_interactions::error::ParseOptionError {
                                field: ::std::convert::From::from(__other),
                                kind: twilight_interactions::error::ParseOptionErrorType::UnknownSubcommand,
                            }
                        )
                    )
                }
            }
        }
    })
}

/// Generate variant match arm
fn variant_match_arm(variant: &ParsedVariant) -> TokenStream {
    let name = &variant.attribute.name;
    let ident = &variant.ident;
    let span = variant.span;

    quote_spanned! {span=>
        #name => {
            let __input = match ::twilight_interactions::command::CommandInputData::from_option(__opt.value, __data.resolved.as_deref()) {
                Ok(__value) => __value,
                Err(__error) => return ::std::result::Result::Err(
                    ::twilight_interactions::error::ParseError::Option(
                        ::twilight_interactions::error::ParseOptionError {
                            field: ::std::convert::From::from(#name),
                            kind: __error,
                        }
                    )
                )
            };

            Ok(Self::#ident(
                ::twilight_interactions::command::CommandModel::from_interaction(__input)?
            ))
        }
    }
}

use crate::modal::parse::StructField;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, FieldsNamed};

/// Implementation of `ModalModel` derive macro
pub fn impl_modal_model(
    input: DeriveInput,
    fields: Option<FieldsNamed>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let fields = match fields {
        Some(fields) => StructField::from_fields(fields)?,
        None => Vec::new(),
    };

    let field_unknown = field_unknown();
    let fields_init = fields.iter().map(field_init);
    let fields_match_arms = fields.iter().map(field_match_arm);
    let fields_constructor = fields.iter().map(field_constructor);

    Ok(quote! {
        impl #generics ::twilight_interactions::modal::ModalModel for #ident #generics #where_clause {
            fn from_interaction(
                __data: ::twilight_interactions::modal::ModalInputData,
            ) -> ::std::result::Result<Self, ::twilight_interactions::error::ParseError> {
                #(#fields_init)*

                for __component in __data.components.into_iter().flat_map(|action_row| action_row.components) {
                    match &*__component.custom_id {
                        #(#fields_match_arms,)*
                        __other => #field_unknown
                    }
                }

                ::std::result::Result::Ok(Self { #(#fields_constructor),* })
            }
        }
    })
}

/// Generate field initialization variables
fn field_init(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    quote!(let mut #ident = None;)
}

/// Generate field match arm
fn field_match_arm(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let span = field.span;

    let custom_id = field.attributes.custom_id_default(ident.to_string());

    quote_spanned! {span=>
        #custom_id => {
            // TODO: Maybe validate type is text input?
            #ident = __component.value;
        }
    }
}

/// Generate field constructor
fn field_constructor(field: &StructField) -> TokenStream {
    let ident = &field.ident;
    let ident_str = ident.to_string();

    match field.kind {
        crate::modal::parse::FieldType::Required => quote! {
            #ident: match #ident {
                Some(__value) => __value,
                None => return Err(::twilight_interactions::error::ParseError::Option(
                    ::twilight_interactions::error::ParseOptionError {
                        field: ::std::convert::From::from(#ident_str),
                        kind: ::twilight_interactions::error::ParseOptionErrorType::RequiredField
                }))
            }
        },
        crate::modal::parse::FieldType::Optional => quote!(#ident),
    }
}

/// Generate unknown field match arm
fn field_unknown() -> TokenStream {
    quote! {
        return ::std::result::Result::Err(
            ::twilight_interactions::error::ParseError::Option(
                ::twilight_interactions::error::ParseOptionError {
                    field: ::std::convert::From::from(__other),
                    kind: ::twilight_interactions::error::ParseOptionErrorType::UnknownField,
            })
        )
    }
}

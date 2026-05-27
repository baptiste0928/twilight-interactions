use crate::modal::parse::{text_input_style, StructField, TypeAttribute};
use crate::parse::syntax::{find_attr, optional};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{DeriveInput, Error, FieldsNamed, Result};

/// Implementation of `CreateModal` derive macro
pub fn impl_create_modal(input: DeriveInput, fields: FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &generics.where_clause;
    let fields = StructField::from_fields(fields)?;

    let capacity = fields.len();
    if !(1..=5).contains(&capacity) {
        return Err(Error::new_spanned(
            input,
            "modal can have at most five fields",
        ));
    }

    let (attributes, attr_span) = match find_attr(&input.attrs, "modal") {
        Some(attr) => (TypeAttribute::parse(attr)?, attr.span()),
        None => {
            return Err(Error::new_spanned(
                input,
                "missing required #[modal(...)] attribute",
            ))
        }
    };

    let title = match attributes.title {
        Some(title) => String::from(title),
        None => return Err(Error::new(attr_span, "missing required attribute `title`")),
    };
    let title_expr = string_field(&title);

    let field_components = fields
        .iter()
        .map(field_component)
        .collect::<Result<Vec<_>>>()?;

    // TODO: Validate custom id length?
    Ok(quote! {
        impl #generics ::twilight_interactions::modal::CreateModal for #ident #generics #where_clause {
            fn create_modal(__custom_id: ::std::string::String) -> ::twilight_interactions::modal::ModalData {
                let mut __modal_components = ::std::vec::Vec::with_capacity(#capacity);

                #(#field_components)*

                ::twilight_interactions::modal::ModalData {
                    custom_id: __custom_id,
                    title: #title_expr,
                    components: __modal_components,
                }
            }
        }
    })
}

/// Generate field component code, including action row wrapper
fn field_component(field: &StructField) -> Result<TokenStream> {
    let span = field.span;

    // TODO: Should custom id have a default?
    let custom_id = string_field(&field.attributes.custom_id_default(field.ident.to_string()));
    let label = string_field(&String::from(field.attributes.label.clone()));
    let max_length = optional(field.attributes.max_length);
    let min_length = optional(field.attributes.min_length);
    let placeholder = optional(
        field
            .attributes
            .placeholder
            .clone()
            .map(|placeholder| string_field(&String::from(placeholder))),
    );
    let required = field.kind.required();
    let style = text_input_style(&field.attributes.style);
    let value = optional(
        field
            .attributes
            .value
            .clone()
            .map(|value| string_field(&String::from(value))),
    );

    Ok(quote_spanned! {span => {
        __modal_components.push(::twilight_model::channel::message::component::Component::ActionRow(
            ::twilight_model::channel::message::component::ActionRow {
                components: ::std::vec![
                    ::twilight_model::channel::message::component::Component::TextInput(
                        ::twilight_model::channel::message::component::TextInput {
                            custom_id: #custom_id,
                            label: #label,
                            max_length: #max_length,
                            min_length: #min_length,
                            placeholder: #placeholder,
                            required: ::std::option::Option::Some(#required),
                            style: #style,
                            value: #value,
                        }
                    )
                ]
            }
        ))
    }})
}

fn string_field(string: &str) -> TokenStream {
    quote! {<::std::string::String as ::std::convert::From<&'static str>>::from(#string)}
}

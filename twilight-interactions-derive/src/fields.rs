//! Parsing of struct fields

use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Result, Type};

use crate::attributes::{find_attr, FieldAttribute};

/// Parsed struct field
pub(crate) struct StructField {
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) ty: Type,
    pub(crate) raw_attrs: Vec<Attribute>,
    pub(crate) attributes: FieldAttribute,
    pub(crate) kind: FieldType,
}

/// Type of a parsed struct field
pub(crate) enum FieldType {
    Required,
    Optional,
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    pub(crate) fn from_field(field: syn::Field) -> Result<Self> {
        let (kind, ty) = match extract_option(&field.ty) {
            Some(ty) => (FieldType::Optional, ty),
            None => (FieldType::Required, field.ty.clone()),
        };

        let attributes = match find_attr(&field.attrs, "command") {
            Some(attr) => FieldAttribute::parse(attr)?,
            None => FieldAttribute::default(),
        };

        Ok(Self {
            span: field.span(),
            ident: field.ident.unwrap(),
            ty,
            raw_attrs: field.attrs,
            attributes,
            kind,
        })
    }

    /// Parse [`syn::FieldsNamed`] as a [`Vec<StructField>`]
    pub(crate) fn from_fields(fields: syn::FieldsNamed) -> Result<Vec<Self>> {
        fields.named.into_iter().map(Self::from_field).collect()
    }
}

impl FieldType {
    pub(crate) fn required(&self) -> bool {
        match self {
            FieldType::Required => true,
            FieldType::Optional => false,
        }
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

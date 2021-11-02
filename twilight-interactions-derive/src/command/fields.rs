//! Parsing of struct fields

use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Result, Type};

use crate::parse::{extract_option, find_attr};

use super::attributes::FieldAttribute;

/// Parsed struct field
pub struct StructField {
    pub span: Span,
    pub ident: Ident,
    pub ty: Type,
    pub raw_attrs: Vec<Attribute>,
    pub attributes: FieldAttribute,
    pub kind: FieldType,
}

/// Type of a parsed struct field
pub enum FieldType {
    Required,
    Optional,
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    pub fn from_field(field: syn::Field) -> Result<Self> {
        let (kind, ty) = match extract_option(&field.ty) {
            Some(ty) => (FieldType::Optional, ty),
            None => (FieldType::Required, field.ty.clone()),
        };

        let attributes = match find_attr(&field.attrs, "command") {
            Some(attr) => FieldAttribute::parse(attr)?,
            None => FieldAttribute::default(),
        };

        Ok(Self {
            span: field.ty.span(),
            ident: field.ident.unwrap(),
            ty,
            raw_attrs: field.attrs,
            attributes,
            kind,
        })
    }

    /// Parse [`syn::FieldsNamed`] as a [`Vec<StructField>`]
    pub fn from_fields(fields: syn::FieldsNamed) -> Result<Vec<Self>> {
        fields.named.into_iter().map(Self::from_field).collect()
    }
}

impl FieldType {
    pub fn required(&self) -> bool {
        match self {
            FieldType::Required => true,
            FieldType::Optional => false,
        }
    }
}

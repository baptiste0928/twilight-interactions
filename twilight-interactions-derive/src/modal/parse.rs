use crate::parse::attribute::{NamedAttrs, ParseAttribute, ParseSpanned};
use crate::parse::parsers::LengthValidatedString;
use crate::parse::syntax::{extract_generic, find_attr};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Lit, Result, Type};

/// Parsed struct field
pub struct StructField {
    pub span: Span,
    pub ident: Ident,
    // TODO: Somehow check that this type is String or introduce something like ComponentModel<TextInput> trait
    pub ty: Type,
    pub raw_attrs: Vec<Attribute>,
    pub attributes: FieldAttribute,
    pub kind: FieldType,
}

/// Type of a parsed struct field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Optional,
    Required,
}

impl FieldType {
    pub fn required(&self) -> bool {
        match self {
            Self::Required => true,
            Self::Optional => false,
        }
    }
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    pub fn from_field(field: syn::Field) -> Result<Self> {
        let (kind, ty) = match extract_generic(&field.ty, "Option") {
            Some(ty) => (FieldType::Optional, ty),
            None => (FieldType::Required, field.ty.clone()),
        };

        let attributes = match find_attr(&field.attrs, "modal") {
            Some(attr) => FieldAttribute::parse(attr)?,
            None => {
                return Err(Error::new_spanned(
                    field,
                    "expected struct field to have modal attribute",
                ))
            }
        };

        let Some(ident) = field.ident else {
            return Err(Error::new_spanned(
                field,
                "expected struct field to have an identifier",
            ));
        };

        Ok(StructField {
            span: field.ty.span(),
            ident,
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

/// Parsed type attribute
pub struct TypeAttribute {
    pub title: Option<LengthValidatedString<1, 45>>,
    pub custom_id: Option<LengthValidatedString<1, 100>>,
}

impl TypeAttribute {
    const VALID_ATTRIBUTES: &'static [&'static str] = &["title", "custom_id"];

    pub fn parse(attr: &Attribute) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, Self::VALID_ATTRIBUTES)?;

        Ok(Self {
            title: parser.optional("title")?,
            custom_id: parser.optional("custom_id")?,
        })
    }
}

/// Parsed field attribute
pub struct FieldAttribute {
    pub label: LengthValidatedString<1, 45>,
    pub custom_id: Option<LengthValidatedString<1, 100>>,
    pub style: TextInputStyle,
    pub value: Option<LengthValidatedString<1, 4000>>,
    pub placeholder: Option<LengthValidatedString<1, 1000>>,
    // TODO: Is this validated somehow?
    pub min_length: Option<u16>,
    pub max_length: Option<u16>,
}

impl FieldAttribute {
    const VALID_ATTRIBUTES: &'static [&'static str] = &[
        "label",
        "custom_id",
        "style",
        "value",
        "placeholder",
        "min_length",
        "max_length",
    ];

    pub fn parse(attr: &Attribute) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, Self::VALID_ATTRIBUTES)?;

        Ok(Self {
            label: parser.required("label")?,
            custom_id: parser.optional("custom_id")?,
            style: parser.required("style")?,
            value: parser.optional("value")?,
            placeholder: parser.optional("placeholder")?,
            min_length: parser.optional("min_length")?,
            max_length: parser.optional("max_length")?,
        })
    }

    pub fn custom_id_default(&self, default: String) -> String {
        match &self.custom_id {
            Some(custom_id) => custom_id.clone().into(),
            None => default,
        }
    }
}

pub enum TextInputStyle {
    Short,
    Paragraph,
}

impl ParseAttribute for TextInputStyle {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;

        match spanned.inner.as_str() {
            "short" => Ok(TextInputStyle::Short),
            "paragraph" => Ok(TextInputStyle::Paragraph),
            invalid => Err(Error::new(
                spanned.span,
                format!(
                    "`{invalid}` is not a valid text input style. \
                    Allowed values are `short` and `paragraph`"
                ),
            )),
        }
    }
}

/// Convert a [`TextInputStyle`] into a [`TokenStream`]
pub fn text_input_style(style: &TextInputStyle) -> TokenStream {
    match style {
        TextInputStyle::Short => {
            quote!(::twilight_model::channel::message::component::TextInputStyle::Short)
        }
        TextInputStyle::Paragraph => {
            quote!(::twilight_model::channel::message::component::TextInputStyle::Paragraph)
        }
    }
}

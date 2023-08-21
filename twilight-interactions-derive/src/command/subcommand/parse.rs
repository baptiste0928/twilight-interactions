use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Error, Fields, Result, Type, TypePath, Variant};

use crate::parse::{
    attribute::NamedAttrs,
    parsers::{CommandDescription, CommandName, FunctionPath},
    syntax::find_attr,
};

/// Parsed enum variant
pub struct ParsedVariant {
    pub span: Span,
    pub ident: Ident,
    pub attribute: VariantAttribute,
    pub inner: TypePath,
}

impl ParsedVariant {
    /// Parse an iterator of syn [`Variant`].
    pub fn from_variants(
        variants: impl IntoIterator<Item = Variant>,
        input_span: Span,
    ) -> Result<Vec<Self>> {
        let variants: Vec<_> = variants.into_iter().collect();

        if variants.is_empty() {
            return Err(Error::new(
                input_span,
                "enum must have at least one variant",
            ));
        }

        variants.into_iter().map(Self::from_variant).collect()
    }

    /// Parse a single syn [`Variant`].
    fn from_variant(variant: Variant) -> Result<Self> {
        let span = variant.span();
        let fields = match variant.fields {
            Fields::Unnamed(fields) => fields,
            _ => return Err(Error::new(span, "variant must be an unnamed variant")),
        };

        if fields.unnamed.len() != 1 {
            return Err(Error::new(
                span,
                "variant must have exactly one unnamed field",
            ));
        }

        let inner = match &fields.unnamed[0].ty {
            // Safety: len is checked above
            Type::Path(ty) => ty.clone(),
            other => {
                return Err(Error::new(
                    other.span(),
                    "unsupported type, expected a type path",
                ))
            }
        };

        let attribute = match find_attr(&variant.attrs, "command") {
            Some(attr) => VariantAttribute::parse(attr)?,
            None => {
                return Err(Error::new(
                    span,
                    "missing required #[command(..)] attribute",
                ))
            }
        };

        Ok(Self {
            span,
            ident: variant.ident,
            attribute,
            inner,
        })
    }
}

/// Parsed variant attribute
pub struct VariantAttribute {
    /// Name of the subcommand
    pub name: CommandName,
}

impl VariantAttribute {
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, &["name"])?;

        Ok(Self {
            name: parser.required("name")?,
        })
    }
}

/// Parsed type attribute
pub struct TypeAttribute {
    /// Name of the command
    pub name: CommandName,
    /// Localization dictionary for the command name.
    pub name_localizations: Option<FunctionPath>,
    /// Description of the command
    pub desc: Option<CommandDescription>,
    /// Localization dictionary for the command description.
    pub desc_localizations: Option<FunctionPath>,
    /// Default permissions required for a member to run the command.
    pub default_permissions: Option<FunctionPath>,
    /// Whether the command is available in DMs.
    pub dm_permission: Option<bool>,
    /// Whether the command is nsfw.
    pub nsfw: Option<bool>,
}

impl TypeAttribute {
    const VALID_ATTRIBUTES: &'static [&'static str] = &[
        "name",
        "name_localizations",
        "desc",
        "desc_localizations",
        "default_permissions",
        "dm_permission",
        "nsfw",
    ];

    pub fn parse(attr: &Attribute) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, Self::VALID_ATTRIBUTES)?;

        Ok(Self {
            name: parser.required("name")?,
            name_localizations: parser.optional("name_localizations")?,
            desc: parser.optional("desc")?,
            desc_localizations: parser.optional("desc_localizations")?,
            default_permissions: parser.optional("default_permissions")?,
            dm_permission: parser.optional("dm_permission")?,
            nsfw: parser.optional("nsfw")?,
        })
    }
}

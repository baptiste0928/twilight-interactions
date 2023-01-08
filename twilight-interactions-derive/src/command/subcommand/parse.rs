use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Attribute, Error, Fields, Result, Type, TypePath, Variant};

use crate::parse::{find_attr, parse_desc, parse_name, parse_path, NamedAttrs};

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
                "Enum must have at least one variant",
            ));
        }

        variants.into_iter().map(Self::from_variant).collect()
    }

    /// Parse a single syn [`Variant`].
    fn from_variant(variant: Variant) -> Result<Self> {
        let span = variant.span();
        let fields = match variant.fields {
            Fields::Unnamed(fields) => fields,
            _ => return Err(Error::new(span, "Variant must be an unnamed variant")),
        };

        if fields.unnamed.len() != 1 {
            return Err(Error::new(
                span,
                "Variant must have exactly one unnamed field",
            ));
        }

        let inner = match &fields.unnamed[0].ty {
            // Safety: len is checked above
            Type::Path(ty) => ty.clone(),
            other => {
                return Err(Error::new(
                    other.span(),
                    "Unsupported type, expected a type path",
                ))
            }
        };

        let attribute = match find_attr(&variant.attrs, "command") {
            Some(attr) => VariantAttribute::parse(attr)?,
            None => {
                return Err(Error::new(
                    span,
                    "Missing required #[command(..)] attribute",
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
    pub name: String,
}

impl VariantAttribute {
    /// Parse a single [`Attribute`].
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["name"])?;

        let name = match attrs.get("name") {
            Some(val) => parse_name(val)?,
            None => return Err(Error::new(attr.span(), "Missing required attribute `name`")),
        };

        Ok(Self { name })
    }
}

/// Parsed type attribute
pub struct TypeAttribute {
    /// Name of the command
    pub name: String,
    /// Localization dictionary for the command name.
    pub name_localizations: Option<syn::Path>,
    /// Description of the command
    pub desc: Option<String>,
    /// Localization dictionary for the command description.
    pub desc_localizations: Option<syn::Path>,
    /// Default permissions required for a member to run the command.
    pub default_permissions: Option<syn::Path>,
    /// Whether the command is available in DMs.
    pub dm_permission: Option<bool>,
    /// Whether the command is nsfw.
    pub nsfw: Option<bool>,
}

impl TypeAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(
            meta,
            &[
                "name",
                "name_localizations",
                "desc",
                "desc_localizations",
                "default_permissions",
                "dm_permission",
                "nsfw",
            ],
        )?;

        let name = match attrs.get("name") {
            Some(val) => parse_name(val)?,
            None => return Err(Error::new(attr.span(), "Missing required attribute `name`")),
        };
        let name_localizations = attrs
            .get("name_localizations")
            .map(parse_path)
            .transpose()?;
        let desc = attrs.get("desc").map(parse_desc).transpose()?;
        let desc_localizations = attrs
            .get("desc_localizations")
            .map(parse_path)
            .transpose()?;
        let default_permissions = attrs
            .get("default_permissions")
            .map(parse_path)
            .transpose()?;
        let dm_permission = attrs
            .get("dm_permission")
            .map(|v| v.parse_bool())
            .transpose()?;
        let nsfw = attrs
            .get("nsfw")
            .map(|v| v.parse_bool())
            .transpose()?;

        Ok(Self {
            name,
            name_localizations,
            desc,
            desc_localizations,
            default_permissions,
            dm_permission,
            nsfw,
        })
    }
}

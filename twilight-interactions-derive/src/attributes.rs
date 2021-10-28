//! Parsing of command attributes

use std::collections::HashMap;

use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Error, Lit, Meta, MetaNameValue, Result};

/// Find an [`Attribute`] with a specific name
///
/// Returns the first match
pub fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    for attr in attrs {
        if let Some(ident) = attr.path.get_ident() {
            if *ident == name {
                return Some(attr);
            }
        }
    }

    None
}

/// Parsed type attribute
pub(crate) struct TypeAttribute {
    /// Rename the field to the given name
    pub(crate) name: String,
    /// Overwrite the field description
    pub(crate) desc: Option<String>,
    /// Limit to specific channel types
    pub(crate) default_permission: bool,
}

impl TypeAttribute {
    /// Parse a single [`Attribute`]
    pub(crate) fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["name", "desc", "default_permission"])?;

        let name = match attrs.get("name") {
            Some(val) => parse_name(val)?,
            None => return Err(Error::new(attr.span(), "Missing required attribute `name`")),
        };
        let desc = attrs.get("desc").map(parse_description).transpose()?;
        let default_permission = attrs
            .get("default_permission")
            .map(|v| v.parse_bool())
            .unwrap_or(Ok(true))?;

        Ok(Self {
            name,
            desc,
            default_permission,
        })
    }
}

/// Parsed field attribute
#[derive(Default)]
pub(crate) struct FieldAttribute {
    /// Rename the field to the given name
    pub(crate) rename: Option<String>,
    /// Overwrite the field description
    pub(crate) desc: Option<String>,
    // Limit to specific channel types
    pub(crate) channel_types: Vec<ChannelType>,
}

impl FieldAttribute {
    /// Parse a single [`Attribute`]
    pub(crate) fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["rename", "desc", "channel_types"])?;

        let rename = attrs.get("rename").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_description).transpose()?;
        let channel_types = attrs
            .get("channel_types")
            .map(ChannelType::parse_attr)
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            rename,
            desc,
            channel_types,
        })
    }

    pub(crate) fn name_default(&self, default: String) -> String {
        match &self.rename {
            Some(name) => name.clone(),
            None => default,
        }
    }
}

/// Parse command or option name.
fn parse_name(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    match val.chars().count() {
        1..=32 => Ok(val),
        _ => Err(Error::new(span, "Name must be between 1 and 32 characters")),
    }
}

/// Parse command or option description
fn parse_description(val: &AttrValue) -> Result<String> {
    let span = val.span();
    let val = val.parse_string()?;

    match val.chars().count() {
        1..=100 => Ok(val),
        _ => Err(Error::new(
            span,
            "Description must be between 1 and 100 characters",
        )),
    }
}

/// Parse description from #[doc] attributes.
///
/// Only fist attribute is parsed (corresponding to the first line of documentation)
/// https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
pub(crate) fn parse_doc(attrs: &[Attribute], span: Span) -> Result<String> {
    let attr = match find_attr(attrs, "doc") {
        Some(attr) => attr,
        None => {
            return Err(Error::new(
                span,
                "Description is required (documentation comment or `desc` attribute)",
            ))
        }
    };

    let doc = match attr.parse_meta()? {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(inner),
            ..
        }) => inner.value().trim().to_owned(),
        _ => {
            return Err(Error::new(
                attr.span(),
                "Failed to parse documentation attribute",
            ))
        }
    };

    match doc.chars().count() {
        1..=100 => Ok(doc),
        _ => Err(Error::new(
            span,
            "Description must be between 1 and 100 characters",
        )),
    }
}

/// Parsed channel type
pub(crate) enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildNews,
    GuildStore,
    GuildNewsThread,
    GuildPublicThread,
    GuildPrivateThread,
    GuildStageVoice,
}

impl ChannelType {
    /// Parse an [`AttrValue`] string as a [`ChannelType`]
    fn parse_attr(attr: &AttrValue) -> Result<Vec<Self>> {
        let span = attr.span();
        let val = attr.parse_string()?;

        val.split_whitespace()
            .map(|value| ChannelType::parse(value, span))
            .collect()
    }

    /// Parse a single string as a [`ChannelType`]
    fn parse(value: &str, span: Span) -> Result<Self> {
        match value {
            "guild_text" => Ok(Self::GuildText),
            "private" => Ok(Self::Private),
            "guild_voice" => Ok(Self::GuildVoice),
            "group" => Ok(Self::Group),
            "guild_category" => Ok(Self::GuildCategory),
            "guild_news" => Ok(Self::GuildNews),
            "guild_store" => Ok(Self::GuildStore),
            "guild_news_thread" => Ok(Self::GuildNewsThread),
            "guild_public_thread" => Ok(Self::GuildPublicThread),
            "guild_private_thread" => Ok(Self::GuildPrivateThread),
            "guild_stage_voice" => Ok(Self::GuildStageVoice),
            invalid => Err(Error::new(
                span,
                format!("`{}` is not a valid channel type", invalid),
            )),
        }
    }
}

/// Parsed list of named attributes like `#[command(rename = "name")]`.
///
/// Attributes are stored as a HashMap with String keys for fast lookups.
struct NamedAttrs(HashMap<String, AttrValue>);

impl NamedAttrs {
    /// Parse a [`Meta`] into [`NamedAttrs`]
    ///
    /// A list of expected parameters must be provided.
    fn parse(meta: Meta, expected: &[&str]) -> Result<Self> {
        // Ensure there is a list of parameters like `#[command(...)]`
        let list = match meta {
            Meta::List(list) => list,
            _ => return Err(Error::new(meta.span(), "Expected named parameters list")),
        };

        let expected = expected.join(", ");
        let mut values = HashMap::new();

        // Parse each item in parameters list
        for nested in list.nested {
            // Ensure each attribute is a name-value attribute like `rename = "name"`
            let inner = match nested {
                syn::NestedMeta::Meta(Meta::NameValue(item)) => item,
                _ => return Err(Error::new(nested.span(), "Expected named parameter")),
            };

            // Extract name of each attribute as String. It must be a single segment path.
            let key = match inner.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => {
                    return Err(Error::new(
                        inner.path.span(),
                        format!("Invalid parameter name (expected {})", expected),
                    ))
                }
            };

            // Ensure the parsed parameter is expected
            if !expected.contains(&&*key) {
                return Err(Error::new(
                    inner.path.span(),
                    format!("Invalid parameter name (expected {})", expected),
                ));
            }

            values.insert(key, AttrValue(inner.lit));
        }

        Ok(Self(values))
    }

    /// Get a parsed parameter by name
    fn get(&self, name: &str) -> Option<&AttrValue> {
        self.0.get(name)
    }
}

/// Parsed attribute value.
///
/// Wrapper around a [`MetaNameValue`] reference with utility methods.
struct AttrValue(Lit);

impl AttrValue {
    fn span(&self) -> Span {
        self.0.span()
    }

    fn parse_string(&self) -> Result<String> {
        match &self.0 {
            Lit::Str(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Invalid attribute type, expected string",
            )),
        }
    }

    fn parse_bool(&self) -> Result<bool> {
        match &self.0 {
            Lit::Bool(inner) => Ok(inner.value()),
            _ => Err(Error::new(
                self.0.span(),
                "Invalid attribute type, expected boolean",
            )),
        }
    }
}

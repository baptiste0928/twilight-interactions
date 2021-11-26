//! Parsing of command attributes

use proc_macro2::Span;
use syn::{spanned::Spanned, Attribute, Error, Lit, Meta, MetaNameValue, Result};

use crate::parse::{find_attr, AttrValue, NamedAttrs};

/// Parsed type attribute
pub struct TypeAttribute {
    /// Rename the field to the given name
    pub name: Option<String>,
    /// Overwrite the field description
    pub desc: Option<String>,
    /// Whether the type is partial
    pub partial: bool,
    /// Limit to specific channel types
    pub default_permission: bool,
}

impl TypeAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(meta, &["name", "desc", "partial", "default_permission"])?;

        let name = attrs.get("name").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_description).transpose()?;
        let partial = attrs
            .get("partial")
            .map(|v| v.parse_bool())
            .transpose()?
            .unwrap_or(false);
        let default_permission = attrs
            .get("default_permission")
            .map(|v| v.parse_bool())
            .transpose()?
            .unwrap_or(true);

        Ok(Self {
            name,
            desc,
            partial,
            default_permission,
        })
    }
}

/// Parsed field attribute
#[derive(Default)]
pub struct FieldAttribute {
    /// Rename the field to the given name
    pub rename: Option<String>,
    /// Overwrite the field description
    pub desc: Option<String>,
    /// Whether the command supports autocomplete
    pub autocomplete: bool,
    /// Limit to specific channel types
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted
    pub min_value: Option<CommandOptionValue>,
}

impl FieldAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(
            meta,
            &[
                "rename",
                "desc",
                "autocomplete",
                "channel_types",
                "max_value",
                "min_value",
            ],
        )?;

        let rename = attrs.get("rename").map(parse_name).transpose()?;
        let desc = attrs.get("desc").map(parse_description).transpose()?;
        let autocomplete = attrs
            .get("autocomplete")
            .map(|val| val.parse_bool())
            .transpose()?
            .unwrap_or_default();
        let channel_types = attrs
            .get("channel_types")
            .map(ChannelType::parse_attr)
            .transpose()?
            .unwrap_or_default();
        let max_value = attrs
            .get("max_value")
            .map(CommandOptionValue::parse_attr)
            .transpose()?;
        let min_value = attrs
            .get("min_value")
            .map(CommandOptionValue::parse_attr)
            .transpose()?;

        Ok(Self {
            rename,
            desc,
            autocomplete,
            channel_types,
            max_value,
            min_value,
        })
    }

    pub fn name_default(&self, default: String) -> String {
        match &self.rename {
            Some(name) => name.clone(),
            None => default,
        }
    }
}

/// Parse command or option name
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

/// Parsed channel type
pub enum ChannelType {
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

/// Parsed command option value
#[derive(Clone, Copy)]
pub enum CommandOptionValue {
    Integer(i64),
    Number(f64),
}

impl CommandOptionValue {
    /// Parse an [`AttrValue`] as a [`CommandOptionValue`]
    fn parse_attr(attr: &AttrValue) -> Result<Self> {
        match attr.inner() {
            Lit::Int(inner) => Ok(Self::Integer(inner.base10_parse()?)),
            Lit::Float(inner) => Ok(Self::Number(inner.base10_parse()?)),
            _ => Err(Error::new(
                attr.span(),
                "Invalid attribute type, expected integer or float",
            )),
        }
    }
}

/// Parse description from #[doc] attributes.
///
/// Only fist attribute is parsed (corresponding to the first line of documentation)
/// https://doc.rust-lang.org/rustdoc/the-doc-attribute.html
pub fn parse_doc(attrs: &[Attribute], span: Span) -> Result<String> {
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

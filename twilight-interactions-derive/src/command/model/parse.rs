//! Parsing of struct fields and attributes

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Attribute, Error, Lit, Result, Type};

use crate::parse::{
    extract_option, extract_type, find_attr, parse_desc, parse_name, parse_path, AttrValue,
    NamedAttrs,
};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Autocomplete,
    Optional,
    Required,
}

impl StructField {
    /// Parse a [`syn::Field`] as a [`StructField`]
    pub fn from_field(field: syn::Field) -> Result<Self> {
        let (kind, ty) = match extract_option(&field.ty) {
            Some(ty) => match extract_type(&ty, "AutocompleteValue") {
                Some(_) => {
                    return Err(Error::new(
                        ty.span(),
                        "AutocompleteValue cannot be wrapped in `Option<T>`",
                    ))
                }
                None => (FieldType::Optional, ty),
            },
            None => match extract_type(&field.ty, "AutocompleteValue") {
                Some(ty) => (FieldType::Autocomplete, ty),
                None => (FieldType::Required, field.ty.clone()),
            },
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
            Self::Required => true,
            Self::Autocomplete | Self::Optional => false,
        }
    }
}

/// Parsed type attribute
pub struct TypeAttribute {
    /// Whether the model is an autocomplete interaction model.
    pub autocomplete: Option<bool>,
    /// Command name.
    pub name: Option<String>,
    /// Localization dictionary for the command name.
    pub name_localizations: Option<syn::Path>,
    /// Command description.
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
                "autocomplete",
                "name",
                "name_localizations",
                "desc",
                "desc_localizations",
                "default_permissions",
                "dm_permission",
                "nsfw",
            ],
        )?;

        let autocomplete = attrs
            .get("autocomplete")
            .map(|v| v.parse_bool())
            .transpose()?;
        let name = attrs.get("name").map(parse_name).transpose()?;
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
        let nsfw = attrs.get("nsfw").map(|v| v.parse_bool()).transpose()?;

        Ok(Self {
            autocomplete,
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

/// Parsed field attribute
#[derive(Default)]
pub struct FieldAttribute {
    /// Rename the field to the given name
    pub rename: Option<String>,
    /// Localization dictionary for the field name.
    pub name_localizations: Option<syn::Path>,
    /// Overwrite the field description
    pub desc: Option<String>,
    /// Localization dictionary for the command description.
    pub desc_localizations: Option<syn::Path>,
    /// Whether the field supports autocomplete
    pub autocomplete: bool,
    /// Limit to specific channel types
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted
    pub min_value: Option<CommandOptionValue>,
    /// Maximum string length
    pub max_length: Option<u16>,
    /// Minimum string length
    pub min_length: Option<u16>,
}

impl FieldAttribute {
    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let meta = attr.parse_meta()?;
        let attrs = NamedAttrs::parse(
            meta,
            &[
                "rename",
                "name_localizations",
                "desc",
                "desc_localizations",
                "autocomplete",
                "channel_types",
                "max_value",
                "min_value",
                "max_length",
                "min_length",
            ],
        )?;

        let rename = attrs.get("rename").map(parse_name).transpose()?;
        let name_localizations = attrs
            .get("name_localizations")
            .map(parse_path)
            .transpose()?;
        let desc = attrs.get("desc").map(parse_desc).transpose()?;
        let desc_localizations = attrs
            .get("desc_localizations")
            .map(parse_path)
            .transpose()?;
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
        let max_length = attrs
            .get("max_length")
            .map(|val| val.parse_int())
            .transpose()?;
        let min_length = attrs
            .get("min_length")
            .map(|val| val.parse_int())
            .transpose()?;

        Ok(Self {
            rename,
            name_localizations,
            desc,
            desc_localizations,
            autocomplete,
            channel_types,
            max_value,
            min_value,
            max_length,
            min_length,
        })
    }

    pub fn name_default(&self, default: String) -> String {
        match &self.rename {
            Some(name) => name.clone(),
            None => default,
        }
    }
}

/// Parsed channel type
pub enum ChannelType {
    GuildText,
    Private,
    GuildVoice,
    Group,
    GuildCategory,
    GuildAnnouncement,
    GuildStore,
    AnnouncementThread,
    PublicThread,
    PrivateThread,
    GuildStageVoice,
    GuildDirectory,
    GuildForum,
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
            "guild_announcement" | "guild_news" => Ok(Self::GuildAnnouncement),
            "guild_store" => Ok(Self::GuildStore),
            "announcement_thread" | "guild_news_thread" => Ok(Self::AnnouncementThread),
            "public_thread" | "guild_public_thread" => Ok(Self::PublicThread),
            "private_thread" | "guild_private_thread" => Ok(Self::PrivateThread),
            "guild_stage_voice" => Ok(Self::GuildStageVoice),
            "guild_directory" => Ok(Self::GuildDirectory),
            "guild_forum" => Ok(Self::GuildForum),
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

/// Convert a [`ChannelType`] into a [`TokenStream`]
pub fn channel_type(kind: &ChannelType) -> TokenStream {
    match kind {
        ChannelType::GuildText => quote!(::twilight_model::channel::ChannelType::GuildText),
        ChannelType::Private => quote!(::twilight_model::channel::ChannelType::Private),
        ChannelType::GuildVoice => quote!(::twilight_model::channel::ChannelType::GuildVoice),
        ChannelType::Group => quote!(::twilight_model::channel::ChannelType::Group),
        ChannelType::GuildCategory => quote!(::twilight_model::channel::ChannelType::GuildCategory),
        ChannelType::GuildAnnouncement => {
            quote!(::twilight_model::channel::ChannelType::GuildAnnouncement)
        }
        ChannelType::GuildStore => quote!(::twilight_model::channel::ChannelType::GuildStore),
        ChannelType::AnnouncementThread => {
            quote!(::twilight_model::channel::ChannelType::AnnouncementThread)
        }
        ChannelType::PublicThread => {
            quote!(::twilight_model::channel::ChannelType::PublicThread)
        }
        ChannelType::PrivateThread => {
            quote!(::twilight_model::channel::ChannelType::PrivateThread)
        }
        ChannelType::GuildStageVoice => {
            quote!(::twilight_model::channel::ChannelType::GuildStageVoice)
        }
        ChannelType::GuildDirectory => quote!(::twilight_model::channel::ChannelType::GuildDirectory),
        ChannelType::GuildForum => quote!(::twilight_model::channel::ChannelType::GuildForum),
    }
}

/// Convert a [`Option<CommandOptionValue>`] into a [`TokenStream`]
pub fn command_option_value(value: Option<CommandOptionValue>) -> TokenStream {
    match value {
        None => quote!(::std::option::Option::None),
        Some(CommandOptionValue::Integer(inner)) => {
            quote!(::std::option::Option::Some(::twilight_model::application::command::CommandOptionValue::Integer(#inner)))
        }
        Some(CommandOptionValue::Number(inner)) => {
            quote!(::std::option::Option::Some(::twilight_model::application::command::CommandOptionValue::Number(#inner)))
        }
    }
}

/// Convert an [`Option<T>`] into a [`TokenStream`]
pub fn optional<T>(value: Option<T>) -> TokenStream
where
    T: ToTokens,
{
    match value {
        Some(value) => quote!(::std::option::Option::Some(#value)),
        None => quote!(::std::option::Option::None),
    }
}

//! Parsing of struct fields and attributes

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Attribute, Error, Lit, Result, Type};

use crate::parse::{
    attribute::{NamedAttrs, ParseAttribute, ParseSpanned},
    parsers::{CommandDescription, CommandName, FunctionPath},
    syntax::{extract_generic, find_attr},
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
        let (kind, ty) = match extract_generic(&field.ty, "Option") {
            Some(ty) => match extract_generic(&ty, "AutocompleteValue") {
                Some(_) => {
                    return Err(Error::new_spanned(
                        ty,
                        "`AutocompleteValue` cannot be wrapped in `Option<T>`",
                    ))
                }
                None => (FieldType::Optional, ty),
            },
            None => match extract_generic(&field.ty, "AutocompleteValue") {
                Some(ty) => (FieldType::Autocomplete, ty),
                None => (FieldType::Required, field.ty.clone()),
            },
        };

        let attributes = match find_attr(&field.attrs, "command") {
            Some(attr) => FieldAttribute::parse(attr)?,
            None => FieldAttribute::default(),
        };

        let Some(ident) = field.ident else {
            return Err(Error::new_spanned(
                field,
                "expected struct field to have an identifier",
            ));
        };

        Ok(Self {
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
    pub name: Option<CommandName>,
    /// Localization dictionary for the command name.
    pub name_localizations: Option<FunctionPath>,
    /// Command description.
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
        "autocomplete",
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
            autocomplete: parser.optional("autocomplete")?,
            name: parser.optional("name")?,
            name_localizations: parser.optional("name_localizations")?,
            desc: parser.optional("desc")?,
            desc_localizations: parser.optional("desc_localizations")?,
            default_permissions: parser.optional("default_permissions")?,
            dm_permission: parser.optional("dm_permission")?,
            nsfw: parser.optional("nsfw")?,
        })
    }
}

/// Parsed field attribute
#[derive(Default)]
pub struct FieldAttribute {
    /// Rename the field to the given name
    pub rename: Option<CommandName>,
    /// Localization dictionary for the field name.
    pub name_localizations: Option<FunctionPath>,
    /// Overwrite the field description
    pub desc: Option<CommandDescription>,
    /// Localization dictionary for the command description.
    pub desc_localizations: Option<FunctionPath>,
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
    const VALID_ATTRIBUTES: &'static [&'static str] = &[
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
    ];

    /// Parse a single [`Attribute`]
    pub fn parse(attr: &Attribute) -> Result<Self> {
        let mut parser = NamedAttrs::parse(attr, Self::VALID_ATTRIBUTES)?;

        Ok(Self {
            rename: parser.optional("rename")?,
            name_localizations: parser.optional("name_localizations")?,
            desc: parser.optional("desc")?,
            desc_localizations: parser.optional("desc_localizations")?,
            autocomplete: parser.optional("autocomplete")?.unwrap_or_default(),
            channel_types: parser.optional("channel_types")?.unwrap_or_default(),
            max_value: parser.optional("max_value")?,
            min_value: parser.optional("min_value")?,
            max_length: parser.optional("max_length")?,
            min_length: parser.optional("min_length")?,
        })
    }

    pub fn name_default(&self, default: String) -> String {
        match &self.rename {
            Some(name) => name.clone().into(),
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

impl ParseAttribute for Vec<ChannelType> {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;

        spanned
            .inner
            .split_ascii_whitespace()
            .map(|value| ChannelType::parse(value, spanned.span))
            .collect()
    }
}

impl ChannelType {
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
                format!("`{invalid}` is not a valid channel type"),
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

impl ParseAttribute for CommandOptionValue {
    fn parse_attribute(input: Lit) -> Result<Self> {
        match input {
            Lit::Int(inner) => Ok(Self::Integer(inner.base10_parse()?)),
            Lit::Float(inner) => Ok(Self::Number(inner.base10_parse()?)),
            _ => Err(Error::new_spanned(
                input,
                "expected integer or floating point literal",
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
        ChannelType::GuildDirectory => {
            quote!(::twilight_model::channel::ChannelType::GuildDirectory)
        }
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

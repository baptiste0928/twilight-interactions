//! Parsing of user applications related structs.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Lit, Result};

use crate::parse::attribute::{ParseAttribute, ParseSpanned};

/// Parsed interaction context type
pub enum InteractionContextType {
    Guild,
    BotDm,
    PrivateChannel,
}

impl ParseAttribute for Vec<InteractionContextType> {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;

        spanned
            .inner
            .split_ascii_whitespace()
            .map(|value| InteractionContextType::parse(value, spanned.span))
            .collect()
    }
}

impl InteractionContextType {
    fn parse(value: &str, span: Span) -> Result<Self> {
        match value {
            "guild" => Ok(Self::Guild),
            "bot_dm" => Ok(Self::BotDm),
            "private_channel" => Ok(Self::PrivateChannel),
            invalid => Err(Error::new(
                span,
                format!("`{invalid}` is not a valid context type"),
            )),
        }
    }
}

/// Parsed application integration type
pub enum ApplicationIntegrationType {
    GuildInstall,
    UserInstall,
}

impl ParseAttribute for Vec<ApplicationIntegrationType> {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;

        spanned
            .inner
            .split_ascii_whitespace()
            .map(|value| ApplicationIntegrationType::parse(value, spanned.span))
            .collect()
    }
}

impl ApplicationIntegrationType {
    fn parse(value: &str, span: Span) -> Result<Self> {
        match value {
            "guild_install" => Ok(Self::GuildInstall),
            "user_install" => Ok(Self::UserInstall),
            invalid => Err(Error::new(
                span,
                format!("`{invalid}` is not a valid integration type"),
            )),
        }
    }
}

/// Convert a [`InteractionContextType`] into a [`TokenStream`]
pub fn context(kind: &InteractionContextType) -> TokenStream {
    match kind {
        InteractionContextType::Guild => {
            quote!(::twilight_model::application::interaction::InteractionContextType::Guild)
        }
        InteractionContextType::BotDm => {
            quote!(::twilight_model::application::interaction::InteractionContextType::BotDm)
        }
        InteractionContextType::PrivateChannel => {
            quote!(
                ::twilight_model::application::interaction::InteractionContextType::PrivateChannel
            )
        }
    }
}

/// Convert a [`ApplicationIntegrationType`] into a [`TokenStream`]
pub fn integration_type(kind: &ApplicationIntegrationType) -> TokenStream {
    match kind {
        ApplicationIntegrationType::GuildInstall => {
            quote!(::twilight_model::oauth::ApplicationIntegrationType::GuildInstall)
        }
        ApplicationIntegrationType::UserInstall => {
            quote!(::twilight_model::oauth::ApplicationIntegrationType::UserInstall)
        }
    }
}

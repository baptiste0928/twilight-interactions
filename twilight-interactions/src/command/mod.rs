//! Slash command parsing and creation.
//!
//!
//! # Slash commands
//! This crate provides parsing slash command data as typed structs. It also
//! provides a convenient way to register commands from these structs. Derive
//! macros are provided to automatically implement related traits.
//!
//! - Command parsing with the [`CommandModel`] trait.
//! - Command creation with the [`CreateCommand`] trait.
//! - Support for subcommands and subcommand groups.
//! - Command option choices with the [`CommandOption`] and [`CreateOption`]
//!   traits.
//!
//! Read the documentation of these traits for usage examples.
//!
//! ## Example
//! ```
//! use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
//!
//! #[derive(CommandModel, CreateCommand)]
//! #[command(name = "hello", desc = "Say hello")]
//! struct HelloCommand {
//!     /// The message to send.
//!     message: String,
//!     /// The user to send the message to.
//!     user: Option<ResolvedUser>,
//! }
//! ```
//!
//! ## Localization
//! Localization of names and descriptions of slash commands is supported
//! using the `name_localizations` and `desc_localizations` attributes on
//! applicable structs.
//!
//! The attribute takes a function that returns any type that implements
//! `IntoIterator<Item = (ToString, ToString)>`, where the first tuple element
//! is a valid [locale](https://discord.com/developers/docs/reference#locales)
//! and the second tuple element is the localized value.
//!
//! ```
//! use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser, DescriptionLocalizations};
//!
//! #[derive(CommandModel, CreateCommand)]
//! #[command(name = "hello", desc_localizations = "hello_desc")]
//! struct HelloCommand;
//!
//! pub fn hello_desc() -> DescriptionLocalizations {
//!     DescriptionLocalizations::new("Say hello", [("fr", "Dis bonjour"), ("de", "Sag Hallo")])
//! }
//! ```
//!
//! See the documentation of the traits to see where these attributes can be
//! used.
//!
//! ## Supported types
//! The [`CommandOption`] and [`CreateOption`] traits are implemented for the
//! following types:
//!
//! | Command option type | Provided implementations                       |
//! |---------------------|------------------------------------------------|
//! | `STRING`            | [`String`], [`Cow`]                            |
//! | `INTEGER`           | [`i64`]                                        |
//! | `NUMBER`            | [`f64`]                                        |
//! | `BOOLEAN`           | [`bool`]                                       |
//! | `USER`              | [`ResolvedUser`], [`User`], [`Id<UserMarker>`] |
//! | `CHANNEL`           | [`InteractionChannel`], [`Id<ChannelMarker>`]  |
//! | `ROLE`              | [`Role`], [`Id<RoleMarker>`]                   |
//! | `MENTIONABLE`       | [`ResolvedMentionable`], [`Id<GenericMarker>`] |
//! | `ATTACHMENT`        | [`Attachment`], [`Id<AttachmentMarker>`]       |
//!
//! [`from_interaction`]: CommandModel::from_interaction
//!
//! [`Cow`]: std::borrow::Cow
//! [`User`]: twilight_model::user::User
//! [`Id<UserMarker>`]: twilight_model::id::Id
//! [`InteractionChannel`]: twilight_model::application::interaction::application_command::InteractionChannel
//! [`Id<ChannelMarker>`]: twilight_model::id::Id
//! [`Role`]: twilight_model::guild::Role
//! [`Id<RoleMarker>`]: twilight_model::id::Id
//! [`Id<GenericMarker>`]: twilight_model::id::Id
//! [`Attachment`]: twilight_model::channel::Attachment
//! [`Id<AttachmentMarker>`]: twilight_model::id::Id

mod command_model;
mod create_command;
mod localizations;

#[doc(hidden)]
pub mod internal;

pub use command_model::{
    AutocompleteValue, CommandInputData, CommandModel, CommandOption, ResolvedMentionable,
    ResolvedUser,
};
pub use create_command::{ApplicationCommandData, CreateCommand, CreateOption};
pub use localizations::{DescriptionLocalizations, NameLocalizations};
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use twilight_interactions_derive::{CommandModel, CommandOption, CreateCommand, CreateOption};

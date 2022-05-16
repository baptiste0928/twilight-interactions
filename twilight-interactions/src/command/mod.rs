//! Slash command parsing and creation.
//!
//!
//! # Slash commands
//! This crate provide parsing slash command data as typed structs. It also provide a convenient
//! way to register commands from these structs. Derive macros are provided to automatically
//! implement related traits.
//!
//! - Command parsing with the [`CommandModel`] trait.
//! - Command creation with the [`CreateCommand`] trait.
//! - Support for subcommands and subcommand groups.
//! - Command option choices with the [`CommandOption`] and [`CreateOption`] traits.
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
//! Localization of names and description of slash command names is supported
//! using the `name_localizations` and `desc_localizations` attributes on
//! applicable structs.
//!
//! The attribute takes a function that return any type that implement
//! `IntoIterator<Item = (ToString, ToString)>`, where the first tuple element
//! is a valid [locale](https://discord.com/developers/docs/reference#locales),
//! and the second tuple element is the localized value.
//!
//! ```
//! use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
//!
//! #[derive(CommandModel, CreateCommand)]
//! #[command(name = "hello", desc = "Say hello", desc_localizations = "hello_desc")]
//! struct HelloCommand;
//!
//! pub fn hello_desc() -> [(&'static str, &'static str); 2] {
//!     [("fr", "Dis bonjour"), ("de", "Sag Hallo")]
//! }
//! ```
//!
//! See the traits documentation to see where these attributes can be used.
//!
//! ## Supported types
//! The [`CommandOption`] and [`CreateOption`] traits are implemented for the following types:
//!
//! | Command option type | Provided implementations               |
//! |---------------------|----------------------------------------|
//! | `STRING`            | [`String`], [`Cow`]                    |
//! | `INTEGER`           | [`i64`]                                |
//! | `NUMBER`            | [`Number`], [`f64`]                    |
//! | `BOOLEAN`           | [`bool`]                               |
//! | `USER`              | [`ResolvedUser`], [`User`], [`UserId`] |
//! | `CHANNEL`           | [`InteractionChannel`], [`ChannelId`]  |
//! | `ROLE`              | [`Role`], [`RoleId`]                   |
//! | `MENTIONABLE`       | [`GenericId`]                          |
//! | `ATTACHMENT`        | [`Attachment`], [`AttachmentId`]       |
//!
//! [`from_interaction`]: CommandModel::from_interaction
//!
//! [`Cow`]: std::borrow::Cow
//! [`Number`]: twilight_model::application::command::Number
//! [`User`]: twilight_model::user::User
//! [`UserId`]: twilight_model::id::UserId
//! [`InteractionChannel`]: twilight_model::application::interaction::application_command::InteractionChannel
//! [`ChannelId`]: twilight_model::id::ChannelId
//! [`Role`]: twilight_model::guild::Role
//! [`RoleId`]: twilight_model::id::RoleId
//! [`GenericId`]: twilight_model::id::GenericId
//! [`Attachment`]: twilight_model::channel::Attachment
//! [`AttachmentId`]: twilight_model::id::AttachmentId

mod command_model;
mod create_command;

#[doc(hidden)]
pub mod internal;

pub use command_model::{CommandInputData, CommandModel, CommandOption, ResolvedUser};
pub use create_command::{ApplicationCommandData, CreateCommand, CreateOption};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use twilight_interactions_derive::{CommandModel, CommandOption, CreateCommand, CreateOption};

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
//! Read the documentation of the [`CommandModel`] and [`CreateCommand`] traits
//! for more information and the complete list of supported attributes.
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
//! For a more complete example, see the [`examples/xkcd-bot`] directory in the
//! library repository.
//!
//! [`examples/xkcd-bot`]:
//!     https://github.com/baptiste0928/twilight-interactions/tree/main/examples/xkcd-bot
//!
//! ## Localization
//! Command names and descriptions can be localized using the
//! `name_localizations` and `desc_localizations` attributes on the command
//! structs and fields.
//!
//! - For command names, you should provide the `name` attribute with the
//!   default command name, and `name_localizations` with the name of a function
//!   that returns a [`NameLocalizations`] struct.
//!
//! - For description, you should only provide the `name_localizations`
//!   attribute with the name of a function that returns a [`DescLocalizations`]
//!   struct.
//!
//!   These structs take a list of tuples, where the first tuple element is a
//!   valid [Discord locale] and the second tuple element is the localized
//!   value.
//!
//! [Discord locale]: https://discord.com/developers/docs/reference#locales
//!
//! ```
//! use twilight_interactions::command::{
//!     CommandModel, CreateCommand, ResolvedUser, NameLocalizations, DescLocalizations
//! };
//!
//! #[derive(CommandModel, CreateCommand)]
//! #[command(
//!     name = "hello",
//!     name_localizations = "hello_name",
//!     desc_localizations = "hello_desc"
//! )]
//! struct HelloCommand;
//!
//! pub fn hello_name() -> NameLocalizations {
//!     NameLocalizations::new([("fr", "bonjour"), ("de", "hallo")])
//! }
//!
//! pub fn hello_desc() -> DescLocalizations {
//!     DescLocalizations::new("Say hello", [("fr", "Dis bonjour"), ("de", "Sag Hallo")])
//! }
//! ```
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
//! Option choices are supported for the `STRING`, `INTEGER` and `NUMBER` option
//! types. See the [`CommandOption`] and [`CreateOption`] traits documentation
//! for more information.
//!
//! [`from_interaction`]: CommandModel::from_interaction
//!
//! [`Cow`]: std::borrow::Cow
//! [`User`]: twilight_model::user::User
//! [`Id<UserMarker>`]: twilight_model::id::Id
//! [`InteractionChannel`]:
//!     twilight_model::application::interaction::InteractionChannel
//! [`Id<ChannelMarker>`]: twilight_model::id::Id
//! [`Role`]: twilight_model::guild::Role
//! [`Id<RoleMarker>`]: twilight_model::id::Id
//! [`Id<GenericMarker>`]: twilight_model::id::Id
//! [`Attachment`]: twilight_model::channel::Attachment
//! [`Id<AttachmentMarker>`]: twilight_model::id::Id

mod command_model;
mod create_command;

#[doc(hidden)]
pub mod internal;

pub use command_model::{
    AutocompleteValue, CommandInputData, CommandModel, CommandOption, ResolvedMentionable,
    ResolvedUser,
};
pub use create_command::{
    ApplicationCommandData, CreateCommand, CreateOption, DescLocalizations, NameLocalizations,
};
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use twilight_interactions_derive::{CommandModel, CommandOption, CreateCommand, CreateOption};

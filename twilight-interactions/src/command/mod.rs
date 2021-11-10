//! Slash command parsing and creation.
//!
//! # Slash commands
//! This crate provide parsing slash command data ([`CommandData`]) as typed structs. It
//! also provide a convenient way to register commands from these structs. Derive macros
//! are provided to automatically implement related traits.
//!
//! ## Command data parsing
//! Parsing is done with the [`CommandModel`] trait which expose the [`from_interaction`]
//! method to parse a [`CommandData`] into a concrete type. A derive macro is available to
//! automatically implement this trait when all field types implements the [`CommandOption`]
//! trait (see below for provided implementations).
//!
//! ### Example usage
//! The following struct correspond to a command that take a required `message` string
//! option and an optional `user` option. The [`ResolvedUser`] type is used to get the
//! optional [`InteractionMember`] associated with the user.
//!
//! ```
//! use twilight_interactions::command::{CommandModel, ResolvedUser};
//!
//! #[derive(CommandModel)]
//! struct HelloCommand {
//!     message: String,
//!     user: Option<ResolvedUser>,
//! }
//! ```
//!
//! This type can then be initialized from a [`CommandData`] using the [`from_interaction`] method.
//!
//! ### Command options validation
//! The [`CommandModel`] only focus on parsing received command data and does not
//! provide a way to perform additional validation. We only support field types
//! that can be validated by Discord.
//!
//! For example, you can use [`User`] in models but not directly [`InteractionMember`] because
//! there is no guarantee that member data will be present when received a `USER` option.
//! The [`ResolvedUser`] type can instead be used to get an optional member object.
//!
//! Because of that, all errors that might occurs when parsing are caused either by invalid
//! data sent by Discord or invalid command registration. It cannot be a bad user input.
//! If you perform additional validation, consider create another type that can be initialized
//! from the raw parsed data.
//!
//! ## Command creation
//! In addition to command data parsing, the [`CreateCommand`] trait and derive macro are
//! provided to register commands corresponding to your models to the Discord API. This
//! is provided by a separate trait because this trait has more advanced requirements
//! that for parsing.
//!
//! The trait can be automatically implemented on struct where all field types implements
//! [`CreateOption`] and have a description (see the example below). The command name must
//! also be provided with the `command` attribute.
//!
//! The derive macro provide a `#[command]` attribute to provide additional information
//! about the command. Refer to the [`CreateCommand`] trait documentation for a full
//! reference of available options.
//!
//! ### Example usage
//! This example is the same as the previous, but additional information has been provided
//! about the command. The same type can derive both [`CommandModel`] and [`CreateCommand`].
//!
//! The example shows two ways to provide a description to the command and its field:
//! - Using documentation comments. Only the first line is used, other are ignored.
//! - Using the `desc` parameter of the `#[command]` attribute.
//!
//! If both are provided, the `desc` parameter will be used.
//!
//! ```
//! use twilight_interactions::command::{CreateCommand, ResolvedUser};
//!
//! #[derive(CreateCommand)]
//! #[command(name = "hello", desc = "Say hello")]
//! struct HelloCommand {
//!     /// The message to send.
//!     message: String,
//!     /// The user to send the message to.
//!     user: Option<ResolvedUser>,
//! }
//! ```
//!
//! An [`ApplicationCommandData`] type corresponding to the command can be obtained using the
//! [`create_command`] method.
//!
//! ## Supported types
//! The [`CommandOption`] and [`CreateOption`] traits are implemented for the following types:
//!
//! | Command option type | Provided implementations               |
//! |---------------------|----------------------------------------|
//! | `STRING`            | [`String`]                             |
//! | `INTEGER`           | [`i64`]                                |
//! | `NUMBER`            | [`Number`], [`f64`]
//! | `BOOLEAN`           | [`bool`]                               |
//! | `USER`              | [`ResolvedUser`], [`User`], [`UserId`] |
//! | `CHANNEL`           | [`InteractionChannel`], [`ChannelId`]  |
//! | `ROLE`              | [`Role`], [`RoleId`]                   |
//! | `MENTIONABLE`       | [`GenericId`]                          |
//! | `SUB_COMMAND`       | Not yet implemented.                   |
//! | `SUB_COMMAND_GROUP` | Not yet implemented.                   |
//!
//! ### Command option choices
//! Command option choices are supported for `STRING`, `INTEGER` and `NUMBER` option types.
//! Derive macros for the [`CommandOption`] and [`CreateOption`] traits are provided to
//! parse command option with choices as enums.
//!
//! ```
//! use twilight_interactions::command::{CommandOption, CreateOption};
//!
//! #[derive(CommandOption, CreateOption)]
//! enum TimeUnit {
//!     #[option(name = "Minute", value = 60)]
//!     Minute,
//!     #[option(name = "Hour", value = 3600)]
//!     Hour,
//!     #[option(name = "Day", value = 86400)]
//!     Day
//! }
//! ```
//!
//! The slash command option type corresponding to the enum is automatically inferred
//! from the `value` parameter. In the previous example, the inferred type would be `INTEGER`.
//!
//! [`from_interaction`]: CommandModel::from_interaction
//! [`create_command`]: CreateCommand::create_command
//!
//! [`CommandData`]: twilight_model::application::interaction::application_command::CommandData
//! [`InteractionMember`]: twilight_model::application::interaction::application_command::InteractionMember
//!
//! [`Number`]: twilight_model::application::command::Number
//! [`User`]: twilight_model::user::User
//! [`UserId`]: twilight_model::id::UserId
//! [`InteractionChannel`]: twilight_model::application::interaction::application_command::InteractionChannel
//! [`ChannelId`]: twilight_model::id::ChannelId
//! [`Role`]: twilight_model::guild::Role
//! [`RoleId`]: twilight_model::id::RoleId
//! [`GenericId`]: twilight_model::id::GenericId

mod command_model;
mod create_command;

pub use command_model::{CommandModel, CommandOption, ResolvedUser};
pub use create_command::{ApplicationCommandData, CommandOptionData, CreateCommand, CreateOption};

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use twilight_interactions_derive::{CommandModel, CommandOption, CreateCommand, CreateOption};

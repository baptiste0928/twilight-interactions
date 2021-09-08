//! # twilight-interactions
//!
//! Set of macros and utilities to work with Discord Interactions using [Twilight](https://twilight.rs/overview.html).
//!
//! **Note:** This crate is not affiliated with the Twilight organization.
//!
//! ## Slash commands
//! One of the primary feature of this crate is parsing slash command data ([`CommandData`]) into typed structure.
//! This is made easy with the use of derive macro to automatically implement the [`CommandModel`] trait on your types.
//! Kind like `serde`, but for Discord slash commands.
//!
//! Each type that implements [`CommandModel`] expose a function to parse it from a [`CommandData`]. This trait can be
//! automatically derived if struct field types implements the [`CommandOption`] trait.
//!
//! ### Example usage
//! The following struct correspond to a command that take a required `message` string option,
//! and an optional `user` option.
//!
//! ```ignore
//! use twilight_interactions::{CommandModel, ResolvedUser};
//!
//! #[derive(CommandModel)]
//! struct HelloCommand {
//!     message: String,
//!     user: Option<ResolvedUser>
//! }
//! ```
//!
//! ### Validating command options
//! Its very common to perform some additional validation on received options. This crate only focus on parsing command data,
//! and therefore the [`CommandOption`] trait is not meant to be implemented on your own types to perform some specific validation.
//!
//! For example, you can use [`Option<InteractionMember>`] in models, but not [`InteractionMember`] as there is
//! not guarantee that member data will be present when receiving a `USER` option.
//!
//! You can adopt a code structure like this if you want to perform additional validation:
//!
//! ```ignore
//! use twilight_interactions::{CommandModel, ResolvedUser};
//! use twilight_model::application::interaction::application_command::{InteractionMember, CommandData};
//!
//! struct HelloCommand {
//!     message: String,
//!     member: InteractionMember
//! }
//!
//! impl HelloCommand {
//!     fn validate(data: CommandData) -> Result<Self, HelloCommandError> {
//!         let parsed = HelloCommandModel::from_interaction(data);
//!         todo!()  // Perform your validations here
//!     }
//! }
//!
//! struct HelloCommandError;
//!
//! #[derive(CommandModel)]
//! struct HelloCommandModel {
//!     pub message: String,
//!     pub member: Option<ResolvedUser>
//! }
//!
//! ```
//!
//! [`CommandData`]: twilight_model::application::interaction::application_command::CommandData
//! [`InteractionMember`]: twilight_model::application::interaction::application_command::InteractionMember
//! [`Option<InteractionMember>`]: twilight_model::application::interaction::application_command::InteractionMember

mod command_model;
mod command_option;
pub mod error;

pub use command_model::CommandModel;
pub use command_option::{CommandOption, ResolvedUser};

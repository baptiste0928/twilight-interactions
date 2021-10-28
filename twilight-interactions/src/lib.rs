//! # twilight-interactions
//!
//! Macros and utilities to make Discord Interactions easy to use with [Twilight](https://twilight.rs/overview.html).
//!
//! **Note:** This crate is not affiliated with the Twilight organization.
//!
//! ## Features
//!
//! ### Slash commands
//! This crate provide a convenient way to parse slash command data on typed structures with derive macro.
//! It also provide a way to register commands to the Discord API using the same models.
//!
//! See the [`command`] module for more information.
//!
//! ## Versioning
//! To facilitate dependencies management, this crate will always use the same major version as the official `twilight` crates.
//!
//! ## Feature flags
//! Additional features can be enabled using [feature flags]:
//!
//! - `derive` (enabled by default): derive macros support.
//! - `http`: create interactions using `twilight-http`.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod command;
pub mod error;

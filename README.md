# twilight-interactions

`twilight-interactions` is a set of macros and utilities to work with Discord Interactions using [`twilight`](https://github.com/twilight-rs/twilight).

**Disclamer:** This crate is currently work in progress and is not affiliated with the [`twilight`](https://github.com/twilight-rs/twilight) project. Breaking changes
may happen at any time before the crate is published on *crates.io*. If you are using it, it is recommended to link to a specific commit in your `Cargo.toml` file to
avoid unwanted breaking change. Any feedback is welcome.

## Features
- Slash command parsing with the `CommandModel` trait
- Slash command with the `CreateCommand` trait
- Advanced slash command option parsing for choices and subcommands (WIP)
- **Derive macros to automatically implement most of provided traits**

## Installing
While this crate is not published on *crates.io*, you should use it as a git dependency.
It is recommended to link to a specific commit to avoid unwanted breaking changes.

```toml
# Cargo.toml
[dependencies]
twilight-interactions = { git = "https://github.com/baptiste0928/twilight-interactions", rev = "commit" }
```

## Example usage

```rust
use twilight_interactions::{CommandModel, ResolvedUser};

#[derive(CommandModel)]
struct HelloCommand {
    message: String,
    user: Option<ResolvedUser>
}
```

> Initial work by [LeSeulArtichaut](https://github.com/LeSeulArtichaut).

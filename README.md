# Twilight interactions

[![Crates.io](https://img.shields.io/crates/v/twilight-interactions)](https://crates.io/crates/twilight-interactions)
[![dependency status](https://deps.rs/repo/github/baptiste0928/twilight-interactions/status.svg)](https://deps.rs/repo/github/baptiste0928/twilight-interactions)
[![docs.rs](https://img.shields.io/docsrs/twilight-interactions)](https://docs.rs/twilight-interactions/)
[![CI](https://github.com/baptiste0928/twilight-interactions/actions/workflows/ci.yaml/badge.svg?event=push)](https://github.com/baptiste0928/twilight-interactions/actions/workflows/ci.yaml)

`twilight-interactions` is a set of macros and utilities to work with Discord Interactions using [`twilight`](https://github.com/twilight-rs/twilight).

**Note:** This crate is not affiliated with the [`twilight`](https://github.com/twilight-rs/twilight) project.

## Features
- **Slash command parsing**: parse interaction data as typed structs using the `CommandModel` macro.
- **Slash command creation**: create commands from your structs with the `CreateCommand` macro. Commands are configured using attributes.

```rust
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello to other members")]
struct HelloCommand {
    /// Message to send
    message: String,
    /// User to send the message to
    user: Option<ResolvedUser>
}
```

## Installing
To install `twilight-interactions`, add the following to your `Cargo.toml`:

```toml
[dependencies]
twilight-interactions = "0.17"
```

The crate's major version follows the version of the official twilight crates.
The current MSRV is `1.89`.

## Documentation

The API documentation is available on docs.rs: [`twilight-interactions` documentation](https://docs.rs/twilight-interactions/).

Examples are available in the [`examples`](https://github.com/baptiste0928/twilight-interactions/tree/main/examples) directory.

## Contributing
There is no particular contribution guidelines, feel free to open a new PR to improve the code. If you want to introduce a new feature, please create an issue before.

*Special thanks to [LeSeulArtichaut](https://github.com/LeSeulArtichaut) who worked the first on this project.*

# Twilight Interactions Examples

This directory contains basic examples of how to use `twilight-interactions` and
the `twilight` ecosystem to build Discord bots.

## xkcd bot

This example shows how to build a simple bot with slash commands to fetch comics
from [xkcd](https://xkcd.com/). It uses regular commands and subcommands, and
shows how to parse and register slash commands using `twilight-interactions`.

You can run this example with:

```sh
$ DISCORD_TOKEN=<your token> cargo run --example xkcd-bot
```

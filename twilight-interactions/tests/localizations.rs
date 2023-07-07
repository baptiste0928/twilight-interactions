use std::collections::HashMap;

use twilight_interactions::command::{
    ApplicationCommandData, CommandModel, CreateCommand, DescriptionLocalizations,
};
use twilight_model::application::command::{CommandOption, CommandOptionType};

fn localize() -> DescriptionLocalizations {
    DescriptionLocalizations::new("fallback", [("en", "english"), ("fr", "french")])
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq)]
#[command(name = "command-desc", desc = "desc")]
struct CommandDesc {
    #[command(desc = "desc")]
    option_one: i64,
    #[command(desc_localizations = "localize")]
    option_two: i64,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq)]
#[command(name = "command-locale", desc_localizations = "localize")]
struct CommandLocale {
    #[command(desc = "desc")]
    option_one: i64,
    #[command(desc_localizations = "localize")]
    option_two: i64,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq)]
#[command(name = "command-group-desc", desc = "desc")]
enum CommandGroupDesc {
    #[command(name = "command-desc")]
    CommandDesc(CommandDesc),
    #[command(name = "command-locale")]
    CommandLocale(CommandLocale),
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq)]
#[command(name = "command-group-locale", desc_localizations = "localize")]
enum CommandGroupLocale {
    #[command(name = "command-desc")]
    CommandDesc(CommandDesc),
    #[command(name = "command-locale")]
    CommandLocale(CommandLocale),
}

fn option(
    name: impl ToString,
    desc: impl ToString,
    locales: Option<HashMap<String, String>>,
) -> CommandOption {
    CommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: None,
        description: desc.to_string(),
        description_localizations: locales,
        kind: CommandOptionType::Integer,
        min_length: None,
        max_length: None,
        max_value: None,
        min_value: None,
        options: None,
        name: name.to_string(),
        name_localizations: None,
        required: Some(true),
    }
}

fn sub_command(
    name: impl ToString,
    desc: impl ToString,
    locales: Option<HashMap<String, String>>,
    options: Vec<CommandOption>,
) -> CommandOption {
    CommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: None,
        description: desc.to_string(),
        description_localizations: locales,
        kind: CommandOptionType::SubCommand,
        min_length: None,
        max_length: None,
        max_value: None,
        min_value: None,
        options: Some(options),
        name: name.to_string(),
        name_localizations: None,
        required: None,
    }
}

fn command(
    name: impl ToString,
    desc: impl ToString,
    locales: Option<HashMap<String, String>>,
    options: Vec<CommandOption>,
    group: bool,
) -> ApplicationCommandData {
    ApplicationCommandData {
        name: name.to_string(),
        name_localizations: None,
        description: desc.to_string(),
        description_localizations: locales,
        options,
        dm_permission: None,
        default_member_permissions: None,
        group,
        nsfw: None,
    }
}

#[test]
fn test_top_level_commands() {
    let options = vec![
        option("option_one", "desc", None),
        option("option_two", "fallback", Some(localize().localizations)),
    ];

    let command_desc = command("command-desc", "desc", None, options.clone(), false);
    let command_locale = command(
        "command-locale",
        "fallback",
        Some(localize().localizations),
        options,
        false,
    );

    assert_eq!(CommandDesc::create_command(), command_desc);
    assert_eq!(CommandLocale::create_command(), command_locale);
}

#[test]
fn test_group_commands() {
    let sub_options = vec![
        option("option_one", "desc", None),
        option("option_two", "fallback", Some(localize().localizations)),
    ];

    let sub_commands = vec![
        sub_command("command-desc", "desc", None, sub_options.clone()),
        sub_command(
            "command-locale",
            "fallback",
            Some(localize().localizations),
            sub_options,
        ),
    ];

    let command_group_desc = command(
        "command-group-desc",
        "desc",
        None,
        sub_commands.clone(),
        true,
    );
    let command_group_locale = command(
        "command-group-locale",
        "fallback",
        Some(localize().localizations),
        sub_commands,
        true,
    );

    assert_eq!(CommandGroupDesc::create_command(), command_group_desc);
    assert_eq!(CommandGroupLocale::create_command(), command_group_locale);
}

use std::collections::HashMap;

use twilight_interactions::command::{
    ApplicationCommandData, CommandInputData, CommandModel, CreateCommand, DescLocalizations,
};
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionType},
        interaction::application_command::{CommandDataOption, CommandOptionValue},
    },
    guild::Permissions,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "one", desc = "Command one")]
struct CommandOne {
    /// An option
    option: String,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "two", desc = "Command two")]
struct CommandTwo {
    /// An option
    option: String,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "three", desc = "Command three")]
struct CommandThree {
    /// An option
    option: String,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "group", desc = "Command group")]
enum SubCommandGroup {
    #[command(name = "two")]
    Two(CommandTwo),
    #[command(name = "three")]
    Three(CommandThree),
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "command",
    desc_localizations = "subcommand_desc",
    default_permissions = "subcommand_permissions"
)]
enum SubCommand {
    #[command(name = "one")]
    One(CommandOne),
    #[command(name = "group")]
    Group(Box<SubCommandGroup>),
}

fn subcommand_desc() -> DescLocalizations {
    DescLocalizations::new("fallback", [("en", "en description")])
}

fn subcommand_permissions() -> Permissions {
    Permissions::empty()
}

#[test]
fn test_subcommand_model() {
    let subcommand_options = vec![CommandDataOption {
        name: "option".into(),
        value: CommandOptionValue::String("test".into()),
    }];

    let command_options = vec![CommandDataOption {
        name: "one".into(),
        value: CommandOptionValue::SubCommand(subcommand_options),
    }];

    let data = CommandInputData {
        options: command_options,
        resolved: None,
    };

    let result = SubCommand::from_interaction(data).unwrap();

    assert_eq!(
        SubCommand::One(CommandOne {
            option: "test".into()
        }),
        result
    );
}

#[test]
fn test_subcommand_group_model() {
    let subcommand_options = vec![CommandDataOption {
        name: "option".into(),
        value: CommandOptionValue::String("test".into()),
    }];

    let subcommand_group_options = vec![CommandDataOption {
        name: "three".into(),
        value: CommandOptionValue::SubCommand(subcommand_options),
    }];

    let command_options = vec![CommandDataOption {
        name: "group".into(),
        value: CommandOptionValue::SubCommandGroup(subcommand_group_options),
    }];

    let data = CommandInputData {
        options: command_options,
        resolved: None,
    };

    let result = SubCommand::from_interaction(data).unwrap();

    assert_eq!(
        SubCommand::Group(Box::new(SubCommandGroup::Three(CommandThree {
            option: "test".into()
        }))),
        result
    );
}

#[test]
fn test_create_subcommand() {
    let command_options = vec![CommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: None,
        description: "An option".into(),
        description_localizations: None,
        kind: CommandOptionType::String,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "option".into(),
        name_localizations: None,
        options: None,
        required: Some(true),
    }];

    let subcommand_group = vec![
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Command two".into(),
            description_localizations: None,
            kind: CommandOptionType::SubCommand,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "two".into(),
            name_localizations: None,
            options: Some(command_options.clone()),
            required: None,
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Command three".into(),
            description_localizations: None,
            kind: CommandOptionType::SubCommand,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "three".into(),
            name_localizations: None,
            options: Some(command_options.clone()),
            required: None,
        },
    ];

    let subcommand = vec![
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Command one".into(),
            description_localizations: None,
            kind: CommandOptionType::SubCommand,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "one".into(),
            name_localizations: None,
            options: Some(command_options),
            required: None,
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Command group".into(),
            description_localizations: None,
            kind: CommandOptionType::SubCommandGroup,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "group".into(),
            name_localizations: None,
            options: Some(subcommand_group),
            required: None,
        },
    ];

    #[allow(deprecated)]
    let expected = ApplicationCommandData {
        name: "command".into(),
        name_localizations: None,
        description: "fallback".into(),
        description_localizations: Some(HashMap::from([("en".into(), "en description".into())])),
        options: subcommand,
        default_member_permissions: Some(Permissions::empty()),
        dm_permission: None,
        group: true,
        nsfw: None,
        contexts: None,
        integration_types: None,
    };

    assert_eq!(SubCommand::create_command(), expected);
}

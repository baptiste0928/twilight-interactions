use std::collections::HashMap;

use twilight_interactions::command::{
    ApplicationCommandData, CommandInputData, CommandModel, CreateCommand,
};
use twilight_model::{
    application::{
        command::{ChoiceCommandOptionData, CommandOption, OptionsCommandOptionData},
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
    desc = "Command",
    desc_localizations = "subcommand_desc",
    default_permissions = "subcommand_permissions"
)]
enum SubCommand {
    #[command(name = "one")]
    One(CommandOne),
    #[command(name = "group")]
    Group(SubCommandGroup),
}

fn subcommand_desc() -> [(&'static str, &'static str); 1] {
    [("en", "Command")]
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
        SubCommand::Group(SubCommandGroup::Three(CommandThree {
            option: "test".into()
        })),
        result
    );
}

#[test]
fn test_create_subcommand() {
    let command_options = vec![CommandOption::String(ChoiceCommandOptionData {
        autocomplete: false,
        choices: vec![],
        description: "An option".into(),
        description_localizations: None,
        max_length: None,
        min_length: None,
        name: "option".into(),
        name_localizations: None,
        required: true,
    })];

    let subcommand_group = vec![
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command two".into(),
            description_localizations: None,
            name: "two".into(),
            name_localizations: None,
            options: command_options.clone(),
        }),
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command three".into(),
            description_localizations: None,
            name: "three".into(),
            name_localizations: None,
            options: command_options.clone(),
        }),
    ];

    let subcommand = vec![
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command one".into(),
            description_localizations: None,
            name: "one".into(),
            name_localizations: None,
            options: command_options,
        }),
        CommandOption::SubCommandGroup(OptionsCommandOptionData {
            description: "Command group".into(),
            description_localizations: None,
            name: "group".into(),
            name_localizations: None,
            options: subcommand_group,
        }),
    ];

    let expected = ApplicationCommandData {
        name: "command".into(),
        name_localizations: None,
        description: "Command".into(),
        description_localizations: Some(HashMap::from([("en".into(), "Command".into())])),
        options: subcommand,
        default_member_permissions: Some(Permissions::empty()),
        dm_permission: None,
        group: true,
    };

    assert_eq!(SubCommand::create_command(), expected);
}

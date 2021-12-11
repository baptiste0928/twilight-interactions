use twilight_interactions::command::{
    ApplicationCommandData, CommandInputData, CommandModel, CreateCommand,
};
use twilight_model::application::{
    command::{ChoiceCommandOptionData, CommandOption, OptionsCommandOptionData},
    interaction::application_command::{CommandDataOption, CommandOptionValue},
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
#[command(name = "command", desc = "Command")]
enum SubCommand {
    #[command(name = "one")]
    One(CommandOne),
    #[command(name = "group")]
    Group(SubCommandGroup),
}

#[test]
fn test_subcommand_model() {
    let subcommand_options = vec![CommandDataOption {
        focused: false,
        name: "option".into(),
        value: CommandOptionValue::String("test".into()),
    }];

    let command_options = vec![CommandDataOption {
        focused: false,
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
        focused: false,
        name: "option".into(),
        value: CommandOptionValue::String("test".into()),
    }];

    let subcommand_group_options = vec![CommandDataOption {
        focused: false,
        name: "three".into(),
        value: CommandOptionValue::SubCommand(subcommand_options),
    }];

    let command_options = vec![CommandDataOption {
        focused: false,
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
        name: "option".into(),
        required: true,
    })];

    let subcommand_group = vec![
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command two".into(),
            name: "two".into(),
            options: command_options.clone(),
        }),
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command three".into(),
            name: "three".into(),
            options: command_options.clone(),
        }),
    ];

    let subcommand = vec![
        CommandOption::SubCommand(OptionsCommandOptionData {
            description: "Command one".into(),
            name: "one".into(),
            options: command_options,
        }),
        CommandOption::SubCommandGroup(OptionsCommandOptionData {
            description: "Command group".into(),
            name: "group".into(),
            options: subcommand_group,
        }),
    ];

    let expected = ApplicationCommandData {
        name: "command".into(),
        description: "Command".into(),
        options: subcommand,
        default_permission: true,
        group: true,
    };

    assert_eq!(SubCommand::create_command(), expected);
}

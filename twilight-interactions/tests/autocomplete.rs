use twilight_interactions::command::{
    AutocompleteValue, CommandInputData, CommandModel, ResolvedUser,
};
use twilight_model::application::{
    command::CommandOptionType,
    interaction::application_command::{CommandDataOption, CommandOptionValue},
};

#[derive(CommandModel, Debug, PartialEq, Eq)]
#[command(autocomplete = true)]
struct DemoCommand {
    user: Option<ResolvedUser>,
    string: AutocompleteValue<String>,
}

#[test]
fn test_autocomplete_model() {
    let options = vec![
        CommandDataOption {
            name: "string".to_string(),
            value: CommandOptionValue::Focused("test".to_string(), CommandOptionType::String),
        },
        CommandDataOption {
            // Should be ignored
            name: "number".to_string(),
            value: CommandOptionValue::Integer(42),
        },
    ];

    let data = CommandInputData {
        options,
        resolved: None,
    };

    let result = DemoCommand::from_interaction(data).unwrap();

    assert_eq!(
        DemoCommand {
            user: None,
            string: AutocompleteValue::Focused("test".to_string())
        },
        result
    )
}

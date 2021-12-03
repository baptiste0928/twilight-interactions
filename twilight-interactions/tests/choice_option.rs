use twilight_interactions::command::{internal::CommandOptionData, CommandOption, CreateOption};
use twilight_model::application::{
    command::{
        ChoiceCommandOptionData, CommandOption as TwilightCommandOption, CommandOptionChoice,
        Number, NumberCommandOptionData,
    },
    interaction::application_command::CommandOptionValue,
};

#[derive(CommandOption, CreateOption, Debug, Clone, Copy, PartialEq, Eq)]
enum ChoiceString {
    #[option(name = "Dog", value = "dog")]
    Dog,
    #[option(name = "Cat", value = "cat")]
    Cat,
    #[option(name = "Crab", value = "crab")]
    Crab,
}

#[derive(CommandOption, CreateOption, Debug, Clone, Copy, PartialEq, Eq)]
enum ChoiceInt {
    #[option(name = "One", value = 1)]
    One,
    #[option(name = "Two", value = 2)]
    Two,
    #[option(name = "Three", value = 3)]
    Three,
}

#[derive(CommandOption, CreateOption, Debug, Clone, Copy, PartialEq, Eq)]
enum ChoiceNumber {
    #[option(name = "One", value = 1.0)]
    One,
    #[option(name = "Half", value = 0.5)]
    Half,
    #[option(name = "Quarter", value = 0.25)]
    Quarter,
}

#[test]
fn test_command_option_string() {
    let parsed = ChoiceString::from_option(CommandOptionValue::String("crab".to_string()), None);
    assert_eq!(parsed, Ok(ChoiceString::Crab));

    let data = CommandOptionData {
        name: "name".to_string(),
        description: "description".to_string(),
        required: false,
        autocomplete: false,
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let command_option = TwilightCommandOption::String(ChoiceCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::String {
                name: "Dog".to_string(),
                value: "dog".to_string(),
            },
            CommandOptionChoice::String {
                name: "Cat".to_string(),
                value: "cat".to_string(),
            },
            CommandOptionChoice::String {
                name: "Crab".to_string(),
                value: "crab".to_string(),
            },
        ],
        description: "description".to_string(),
        name: "name".to_string(),
        required: false,
    });

    assert_eq!(command_option, ChoiceString::create_option(data))
}

#[test]
fn test_command_option_integer() {
    let parsed = ChoiceInt::from_option(CommandOptionValue::Integer(2), None);
    assert_eq!(parsed, Ok(ChoiceInt::Two));

    let data = CommandOptionData {
        name: "name".to_string(),
        description: "description".to_string(),
        required: false,
        autocomplete: false,
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let command_option = TwilightCommandOption::Integer(NumberCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::Int {
                name: "One".to_string(),
                value: 1,
            },
            CommandOptionChoice::Int {
                name: "Two".to_string(),
                value: 2,
            },
            CommandOptionChoice::Int {
                name: "Three".to_string(),
                value: 3,
            },
        ],
        description: "description".to_string(),
        max_value: None,
        min_value: None,
        name: "name".to_string(),
        required: false,
    });

    assert_eq!(command_option, ChoiceInt::create_option(data));
}

#[test]
fn test_command_option_number() {
    let parsed = ChoiceNumber::from_option(CommandOptionValue::Number(Number(0.5)), None);
    assert_eq!(parsed, Ok(ChoiceNumber::Half));

    let data = CommandOptionData {
        name: "name".to_string(),
        description: "description".to_string(),
        required: false,
        autocomplete: false,
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let command_option = TwilightCommandOption::Number(NumberCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::Number {
                name: "One".to_string(),
                value: Number(1.0),
            },
            CommandOptionChoice::Number {
                name: "Half".to_string(),
                value: Number(0.5),
            },
            CommandOptionChoice::Number {
                name: "Quarter".to_string(),
                value: Number(0.25),
            },
        ],
        description: "description".to_string(),
        max_value: None,
        min_value: None,
        name: "name".to_string(),
        required: false,
    });

    assert_eq!(command_option, ChoiceNumber::create_option(data));
}

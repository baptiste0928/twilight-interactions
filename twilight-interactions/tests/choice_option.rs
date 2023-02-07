use std::collections::HashMap;

use twilight_interactions::command::{
    internal::{CommandOptionData, CreateOptionData},
    CommandOption, CreateOption,
};
use twilight_model::application::{
    command::{
        CommandOption as TwilightCommandOption, CommandOptionChoice, CommandOptionChoiceValue,
        CommandOptionType,
    },
    interaction::application_command::CommandOptionValue,
};

#[derive(CommandOption, CreateOption, Debug, Clone, Copy, PartialEq, Eq)]
enum ChoiceString {
    #[option(name = "Dog", name_localizations = "name_dog", value = "dog")]
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

pub fn name_dog() -> [(&'static str, &'static str); 1] {
    [("en", "Dog")]
}

#[test]
fn test_command_option_string() {
    let parsed = ChoiceString::from_option(
        CommandOptionValue::String("crab".to_string()),
        CommandOptionData::default(),
        None,
    );
    assert_eq!(parsed, Ok(ChoiceString::Crab));
    assert_eq!(ChoiceString::Dog.value(), "dog");
    assert_eq!(ChoiceString::Cat.value(), "cat");
    assert_eq!(ChoiceString::Crab.value(), "crab");

    let data = CommandOptionData {
        channel_types: None,
        max_value: None,
        min_value: None,
        max_length: None,
        min_length: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: Some(false),
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: Some(vec![
            CommandOptionChoice {
                name: "Dog".to_string(),
                name_localizations: Some(HashMap::from([("en".to_string(), "Dog".to_string())])),
                value: CommandOptionChoiceValue::String("dog".to_string()),
            },
            CommandOptionChoice {
                name: "Cat".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::String("cat".to_string()),
            },
            CommandOptionChoice {
                name: "Crab".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::String("crab".to_string()),
            },
        ]),
        description: "description".to_string(),
        description_localizations: None,
        kind: CommandOptionType::String,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "name".to_string(),
        name_localizations: None,
        options: None,
        required: Some(false),
    };

    assert_eq!(command_option, ChoiceString::create_option(create_data))
}

#[test]
fn test_command_option_integer() {
    let parsed = ChoiceInt::from_option(
        CommandOptionValue::Integer(2),
        CommandOptionData::default(),
        None,
    );
    assert_eq!(parsed, Ok(ChoiceInt::Two));
    assert_eq!(ChoiceInt::One.value(), 1);
    assert_eq!(ChoiceInt::Two.value(), 2);
    assert_eq!(ChoiceInt::Three.value(), 3);

    let data = CommandOptionData {
        channel_types: None,
        max_value: None,
        min_value: None,
        min_length: None,
        max_length: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: Some(false),
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: Some(vec![
            CommandOptionChoice {
                name: "One".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Integer(1),
            },
            CommandOptionChoice {
                name: "Two".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Integer(2),
            },
            CommandOptionChoice {
                name: "Three".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Integer(3),
            },
        ]),
        description: "description".to_string(),
        description_localizations: None,
        kind: CommandOptionType::Integer,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "name".to_string(),
        name_localizations: None,
        options: None,
        required: Some(false),
    };

    assert_eq!(command_option, ChoiceInt::create_option(create_data));
}

#[test]
fn test_command_option_number() {
    let parsed = ChoiceNumber::from_option(
        CommandOptionValue::Number(0.5),
        CommandOptionData::default(),
        None,
    );
    assert_eq!(parsed, Ok(ChoiceNumber::Half));
    assert_eq!(ChoiceNumber::One.value(), 1.0);
    assert_eq!(ChoiceNumber::Half.value(), 0.5);
    assert_eq!(ChoiceNumber::Quarter.value(), 0.25);

    let data = CommandOptionData {
        channel_types: None,
        max_value: None,
        min_value: None,
        max_length: None,
        min_length: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: Some(false),
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption {
        autocomplete: Some(false),
        channel_types: None,
        choices: Some(vec![
            CommandOptionChoice {
                name: "One".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Number(1.0),
            },
            CommandOptionChoice {
                name: "Half".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Number(0.5),
            },
            CommandOptionChoice {
                name: "Quarter".to_string(),
                name_localizations: None,
                value: CommandOptionChoiceValue::Number(0.25),
            },
        ]),
        description: "description".to_string(),
        description_localizations: None,
        kind: CommandOptionType::Number,
        max_length: None,
        max_value: None,
        min_length: None,
        min_value: None,
        name: "name".to_string(),
        name_localizations: None,
        options: None,
        required: Some(false),
    };

    assert_eq!(command_option, ChoiceNumber::create_option(create_data));
}

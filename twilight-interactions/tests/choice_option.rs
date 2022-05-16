use std::collections::HashMap;

use twilight_interactions::command::{
    internal::{CommandOptionData, CreateOptionData},
    CommandOption, CreateOption,
};
use twilight_model::application::{
    command::{
        ChoiceCommandOptionData, CommandOption as TwilightCommandOption, CommandOptionChoice,
        Number, NumberCommandOptionData,
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
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: false,
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption::String(ChoiceCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::String {
                name: "Dog".to_string(),
                name_localizations: Some(HashMap::from([("en".into(), "Dog".into())])),
                value: "dog".to_string(),
            },
            CommandOptionChoice::String {
                name: "Cat".to_string(),
                name_localizations: None,
                value: "cat".to_string(),
            },
            CommandOptionChoice::String {
                name: "Crab".to_string(),
                name_localizations: None,
                value: "crab".to_string(),
            },
        ],
        description: "description".to_string(),
        description_localizations: None,
        name: "name".to_string(),
        name_localizations: None,
        required: false,
    });

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
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: false,
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption::Integer(NumberCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::Int {
                name: "One".to_string(),
                name_localizations: None,
                value: 1,
            },
            CommandOptionChoice::Int {
                name: "Two".to_string(),
                name_localizations: None,
                value: 2,
            },
            CommandOptionChoice::Int {
                name: "Three".to_string(),
                name_localizations: None,
                value: 3,
            },
        ],
        description: "description".to_string(),
        description_localizations: None,
        max_value: None,
        min_value: None,
        name: "name".to_string(),
        name_localizations: None,
        required: false,
    });

    assert_eq!(command_option, ChoiceInt::create_option(create_data));
}

#[test]
fn test_command_option_number() {
    let parsed = ChoiceNumber::from_option(
        CommandOptionValue::Number(Number(0.5)),
        CommandOptionData::default(),
        None,
    );
    assert_eq!(parsed, Ok(ChoiceNumber::Half));
    assert_eq!(ChoiceNumber::One.value(), 1.0);
    assert_eq!(ChoiceNumber::Half.value(), 0.5);
    assert_eq!(ChoiceNumber::Quarter.value(), 0.25);

    let data = CommandOptionData {
        channel_types: Vec::new(),
        max_value: None,
        min_value: None,
    };
    let create_data = CreateOptionData {
        name: "name".to_string(),
        name_localizations: None,
        description: "description".to_string(),
        description_localizations: None,
        required: false,
        autocomplete: false,
        data,
    };

    let command_option = TwilightCommandOption::Number(NumberCommandOptionData {
        autocomplete: false,
        choices: vec![
            CommandOptionChoice::Number {
                name: "One".to_string(),
                name_localizations: None,
                value: Number(1.0),
            },
            CommandOptionChoice::Number {
                name: "Half".to_string(),
                name_localizations: None,
                value: Number(0.5),
            },
            CommandOptionChoice::Number {
                name: "Quarter".to_string(),
                name_localizations: None,
                value: Number(0.25),
            },
        ],
        description: "description".to_string(),
        description_localizations: None,
        max_value: None,
        min_value: None,
        name: "name".to_string(),
        name_localizations: None,
        required: false,
    });

    assert_eq!(command_option, ChoiceNumber::create_option(create_data));
}

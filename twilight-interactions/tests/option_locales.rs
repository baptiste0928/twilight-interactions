use std::collections::HashMap;

use twilight_interactions::command::{ApplicationCommandData, CreateCommand};
use twilight_model::{
    application::command::{CommandOption, CommandOptionType},
    guild::Permissions,
};

/// A command
#[derive(CreateCommand)]
#[command(name = "demo")]
struct Command {
    /// Option with doc description
    doc_option: String,
    #[command(desc = "option with desc attribute")]
    attr_option: String,
    #[command(desc_localizations = "locale_option")]
    locale_option: String,
}

fn locale_option() -> [(&'static str, &'static str); 2] {
    [("en", "en description"), ("", "fallback description")]
}

#[test]
fn test_option_description() {
    let locales = HashMap::from([("en".into(), "en description".into())]);

    let options = vec![
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Option with doc description".into(),
            description_localizations: None,
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "doc_option".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "option with desc attribute".into(),
            description_localizations: None,
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "attr_option".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "fallback description".into(),
            description_localizations: Some(locales),
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "locale_option".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
    ];

    let expected = ApplicationCommandData {
        name: "demo".into(),
        name_localizations: None,
        description: "A command".into(),
        description_localizations: None,
        options,
        default_member_permissions: None,
        dm_permission: None,
        group: false,
        nsfw: None,
    };

    assert_eq!(Command::create_command(), Ok(expected));
}

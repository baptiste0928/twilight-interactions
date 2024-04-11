use std::{borrow::Cow, collections::HashMap};

use twilight_interactions::command::{
    ApplicationCommandData, CreateCommand, CreateOption, ResolvedUser,
};
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionType, CommandOptionValue},
        interaction::InteractionChannel,
    },
    channel::ChannelType,
    guild::Permissions,
};

/// Demo command for testing purposes
#[derive(CreateCommand, Debug, PartialEq)]
#[command(
    name = "demo",
    name_localizations = "demo_name",
    default_permissions = "demo_permissions",
    dm_permission = false,
    nsfw = true
)]
struct DemoCommand<'a, T>
where
    T: CreateOption,
{
    /// This should be overwritten
    #[command(rename = "member", desc = "A member")]
    user: ResolvedUser,
    /// Some text
    ///
    /// This documentation comment is ignored
    #[command(min_length = 5)]
    text: String,
    /// A number
    #[command(autocomplete = true, max_value = 50.0)]
    number: f64,
    /// A text channel
    #[command(channel_types = "guild_text private")]
    channel: Option<InteractionChannel>,
    /// Generic field
    generic: Option<T>,
    /// More text
    cow: Option<Cow<'a, str>>,
}

fn demo_permissions() -> Permissions {
    Permissions::SEND_MESSAGES
}

fn demo_name() -> [(&'static str, &'static str); 1] {
    [("en", "demo")]
}

#[derive(CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "unit", desc = "Unit command for testing purposes")]
struct UnitCommand;

#[test]
fn test_create_command() {
    let options = vec![
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "A member".into(),
            description_localizations: None,
            kind: CommandOptionType::User,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "member".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Some text".into(),
            description_localizations: None,
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: Some(5),
            min_value: None,
            name: "text".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
        CommandOption {
            autocomplete: Some(true),
            channel_types: None,
            choices: None,
            description: "A number".into(),
            description_localizations: None,
            kind: CommandOptionType::Number,
            max_length: None,
            max_value: Some(CommandOptionValue::Number(50.0)),
            min_length: None,
            min_value: None,
            name: "number".into(),
            name_localizations: None,
            options: None,
            required: Some(true),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: Some(vec![ChannelType::GuildText, ChannelType::Private]),
            choices: None,
            description: "A text channel".into(),
            description_localizations: None,
            kind: CommandOptionType::Channel,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "channel".into(),
            name_localizations: None,
            options: None,
            required: Some(false),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "Generic field".into(),
            description_localizations: None,
            kind: CommandOptionType::Integer,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "generic".into(),
            name_localizations: None,
            options: None,
            required: Some(false),
        },
        CommandOption {
            autocomplete: Some(false),
            channel_types: None,
            choices: None,
            description: "More text".into(),
            description_localizations: None,
            kind: CommandOptionType::String,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            name: "cow".into(),
            name_localizations: None,
            options: None,
            required: Some(false),
        },
    ];

    let name_localizations = HashMap::from([("en".into(), "demo".into())]);

    let expected = ApplicationCommandData {
        name: "demo".into(),
        name_localizations: Some(name_localizations),
        description: "Demo command for testing purposes".into(),
        description_localizations: None,
        options,
        default_member_permissions: Some(Permissions::SEND_MESSAGES),
        dm_permission: Some(false),
        group: false,
        nsfw: Some(true),
    };

    assert_eq!(DemoCommand::<i64>::create_command(), expected);
    assert_eq!(DemoCommand::<i64>::NAME, "demo");
}

#[test]
fn test_unit_create_command() {
    let expected = ApplicationCommandData {
        name: "unit".into(),
        name_localizations: None,
        description: "Unit command for testing purposes".into(),
        description_localizations: None,
        options: vec![],
        default_member_permissions: None,
        dm_permission: None,
        group: false,
        nsfw: None,
    };

    assert_eq!(UnitCommand::create_command(), expected);
    assert_eq!(UnitCommand::NAME, "unit");
}

use twilight_interactions::command::{ApplicationCommandData, CreateCommand, ResolvedUser};
use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, CommandOption,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::ChannelType,
};

/// Demo command for testing purposes
#[derive(CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "demo")]
struct DemoCommand {
    #[command(rename = "member", desc = "A member")]
    /// This should be overwritten
    user: ResolvedUser,
    /// Some text
    ///
    /// This documentation comment is ignored
    text: String,
    /// A text channel
    #[command(channel_types = "guild_text private")]
    channel: Option<InteractionChannel>,
}

#[test]
fn test_create_command() {
    let options = vec![
        CommandOption::User(BaseCommandOptionData {
            description: "A member".into(),
            name: "member".into(),
            required: true,
        }),
        CommandOption::String(ChoiceCommandOptionData {
            description: "Some text".into(),
            name: "text".into(),
            required: true,
            choices: vec![],
        }),
        CommandOption::Channel(ChannelCommandOptionData {
            channel_types: vec![ChannelType::GuildText, ChannelType::Private],
            description: "A text channel".into(),
            name: "channel".into(),
            required: false,
        }),
    ];

    let expected = ApplicationCommandData {
        name: "demo".into(),
        description: "Demo command for testing purposes".into(),
        options,
        default_permission: true,
    };

    assert_eq!(DemoCommand::create_command(), expected);
}

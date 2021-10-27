use twilight_interactions::{
    ApplicationCommandData, CommandOptionData, CreateCommand, CreateOption, ResolvedUser,
};

/// Demo command for testing purposes
#[derive(CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "demo")]
struct DemoCommand {
    #[command(rename = "member", desc = "A member")]
    /// This should be overwritten
    user: ResolvedUser,
    /// Some text
    text: String,
    /// A number
    number: Option<i64>,
}

#[test]
fn test_create_command() {
    let mut options = Vec::new();

    options.push(ResolvedUser::create_option(CommandOptionData {
        name: "member".into(),
        description: "A member".into(),
        required: true,
    }));

    options.push(String::create_option(CommandOptionData {
        name: "text".into(),
        description: "Some text".into(),
        required: true,
    }));

    options.push(i64::create_option(CommandOptionData {
        name: "number".into(),
        description: "A number".into(),
        required: false,
    }));

    let expected = ApplicationCommandData {
        name: "demo".into(),
        description: "Demo command for testing purposes".into(),
        options,
        default_permission: true,
    };

    assert_eq!(DemoCommand::create_command(), expected);
}

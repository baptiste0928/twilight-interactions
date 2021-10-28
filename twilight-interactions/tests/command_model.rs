use twilight_interactions::command::{CommandModel, ResolvedUser};
use twilight_model::{
    application::interaction::application_command::{
        CommandData, CommandDataOption, CommandInteractionDataResolved, CommandOptionValue,
        InteractionMember,
    },
    id::{CommandId, UserId},
    user::User,
};

#[derive(CommandModel, Debug, PartialEq, Eq)]
struct DemoCommand {
    #[command(rename = "member", desc = "test")]
    user: ResolvedUser,
    text: String,
    number: Option<i64>,
}

#[test]
fn test_command_model() {
    let user_id = UserId::new(123).unwrap();
    let options = vec![
        CommandDataOption {
            name: "member".to_string(),
            value: CommandOptionValue::User(user_id),
        },
        CommandDataOption {
            name: "text".into(),
            value: CommandOptionValue::String("hello world".into()),
        },
        CommandDataOption {
            name: "number".into(),
            value: CommandOptionValue::Integer(42),
        },
    ];

    let member = InteractionMember {
        hoisted_role: None,
        id: user_id,
        joined_at: None,
        nick: None,
        premium_since: None,
        roles: vec![],
    };

    let user = User {
        avatar: None,
        bot: false,
        discriminator: 1,
        email: None,
        flags: None,
        id: user_id,
        locale: None,
        mfa_enabled: None,
        name: "someone".into(),
        premium_type: None,
        public_flags: None,
        system: None,
        verified: None,
        accent_color: None,
        banner: None,
    };

    let resolved = CommandInteractionDataResolved {
        channels: Vec::new(),
        members: vec![member.clone()],
        roles: Vec::new(),
        users: vec![user.clone()],
        messages: Vec::new(),
    };

    let data = CommandData {
        id: CommandId::new(123).unwrap(),
        name: "demo".to_string(),
        options,
        resolved: Some(resolved),
    };

    let result = DemoCommand::from_interaction(data).unwrap();

    assert_eq!(
        DemoCommand {
            user: ResolvedUser {
                resolved: user,
                member: Some(member)
            },
            text: "hello world".into(),
            number: Some(42),
        },
        result
    );
}

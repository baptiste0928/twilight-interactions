use twilight_interactions::{CommandModel, ResolvedUser};
use twilight_model::{
    application::interaction::application_command::{
        CommandData, CommandDataOption, CommandInteractionDataResolved, InteractionMember,
    },
    user::User,
};

#[derive(CommandModel, Debug, PartialEq, Eq)]
struct DemoCommand {
    user: ResolvedUser,
    text: String,
    number: Option<i64>,
}

#[test]
fn test_demo_command() {
    let options = vec![
        CommandDataOption::String {
            name: "user".to_string(),
            value: "123".to_string(),
        },
        CommandDataOption::String {
            name: "text".into(),
            value: "hello world".into(),
        },
        CommandDataOption::Integer {
            name: "number".into(),
            value: 42,
        },
    ];

    let member = InteractionMember {
        hoisted_role: None,
        id: 123.into(),
        joined_at: None,
        nick: None,
        premium_since: None,
        roles: vec![],
    };

    let user = User {
        avatar: None,
        bot: false,
        discriminator: "0001".into(),
        email: None,
        flags: None,
        id: 123.into(),
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
        id: 123.into(),
        name: "demo".to_string(),
        options,
        resolved: Some(resolved),
    };

    let result = DemoCommand::from_interaction(data.clone()).unwrap();

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

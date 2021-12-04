use std::{borrow::Cow, collections::HashMap};

use maplit::hashmap;
use twilight_interactions::command::{CommandInputData, CommandModel, ResolvedUser};
use twilight_model::{
    application::interaction::application_command::{
        CommandDataOption, CommandInteractionDataResolved, CommandOptionValue, InteractionMember,
    },
    datetime::Timestamp,
    id::UserId,
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
            focused: false,
        },
        CommandDataOption {
            name: "text".into(),
            value: CommandOptionValue::String("hello world".into()),
            focused: false,
        },
        CommandDataOption {
            name: "number".into(),
            value: CommandOptionValue::Integer(42),
            focused: false,
        },
    ];

    let member = InteractionMember {
        joined_at: Timestamp::from_secs(1609455600).unwrap(),
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
        channels: HashMap::new(),
        members: hashmap! { user_id => member.clone() },
        roles: HashMap::new(),
        users: hashmap! { user_id => user.clone() },
        messages: HashMap::new(),
    };

    let data = CommandInputData {
        options,
        resolved: Some(Cow::Owned(resolved)),
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

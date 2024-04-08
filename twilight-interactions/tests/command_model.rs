use std::{borrow::Cow, collections::HashMap};

use twilight_interactions::command::{
    CommandInputData, CommandModel, CommandOption, ResolvedMentionable, ResolvedUser,
};
use twilight_model::{
    application::interaction::{
        application_command::{CommandDataOption, CommandOptionValue},
        InteractionDataResolved, InteractionMember,
    },
    guild::{MemberFlags, Permissions},
    id::Id,
    user::User,
    util::Timestamp,
};

#[derive(CommandModel, Debug, PartialEq, Eq)]
struct DemoCommand<'a, T>
where
    T: CommandOption,
{
    #[command(rename = "member", desc = "test")]
    user: ResolvedUser,
    text: String,
    number: Option<i64>,
    generic: T,
    cow: Cow<'a, str>,
    mentionable: ResolvedMentionable,
}

#[derive(CommandModel, Debug, PartialEq, Eq)]
struct UnitCommand;

#[test]
fn test_command_model() {
    let user_id = Id::new(123);
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
        CommandDataOption {
            name: "generic".into(),
            value: CommandOptionValue::Integer(0),
        },
        CommandDataOption {
            name: "cow".into(),
            value: CommandOptionValue::String("cow".into()),
        },
        CommandDataOption {
            name: "mentionable".into(),
            value: CommandOptionValue::Mentionable(user_id.cast()),
        },
    ];

    let member = InteractionMember {
        joined_at: Some(Timestamp::from_secs(1609455600).unwrap()),
        nick: None,
        premium_since: None,
        roles: vec![],
        avatar: None,
        communication_disabled_until: None,
        pending: false,
        permissions: Permissions::empty(),
        flags: MemberFlags::empty(),
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
        avatar_decoration: None,
        global_name: None,
    };

    let resolved_user = ResolvedUser {
        resolved: user.clone(),
        member: Some(member.clone()),
    };

    let resolved = InteractionDataResolved {
        channels: HashMap::new(),
        members: HashMap::from([(user_id, member)]),
        roles: HashMap::new(),
        users: HashMap::from([(user_id, user)]),
        messages: HashMap::new(),
        attachments: HashMap::new(),
    };

    let data = CommandInputData {
        options,
        resolved: Some(Cow::Owned(resolved)),
    };

    let result = DemoCommand::from_interaction(data).unwrap();

    assert_eq!(
        DemoCommand {
            user: resolved_user.clone(),
            text: "hello world".into(),
            number: Some(42),
            generic: 0_i64,
            cow: Cow::Borrowed("cow"),
            mentionable: ResolvedMentionable::User(resolved_user)
        },
        result
    );
}

#[test]
fn test_unit_command_model() {
    let data = CommandInputData {
        options: vec![],
        resolved: None,
    };

    let result = UnitCommand::from_interaction(data).unwrap();

    assert_eq!(UnitCommand, result);
}

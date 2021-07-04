use twilight_model::{
    application::{
        command::{CommandOption as ApplicationCommandOption, CommandOptionType},
        interaction::application_command::{
            CommandData, CommandInteractionDataResolved, CommandOptionValue, InteractionChannel,
            InteractionMember,
        },
    },
    guild::Role,
    id::{ChannelId, RoleId, UserId},
    user::User,
};
pub use twilight_slash_proc::SlashCommand;

pub trait SlashCommand: Sized {
    fn from_interaction(data: CommandData) -> Option<Self>;
    fn create_application_command() -> CreateApplicationCommand;
}

pub trait CommandOption: Sized {
    const OPTION_TYPE: CommandOptionType;
    const DEFAULT: Option<Self> = None;
    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self>;
}

pub struct CreateApplicationCommand {
    pub name: String,
    pub description: String,
    pub options: Vec<ApplicationCommandOption>,
    pub default_permission: Option<bool>,
}

impl CommandOption for String {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::String;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::String(s) => Some(s),
            _ => None,
        }
    }
}

impl CommandOption for i64 {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Integer;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Integer(i) => Some(i),
            _ => None,
        }
    }
}

impl CommandOption for bool {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Boolean;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Boolean(b) => Some(b),
            _ => None,
        }
    }
}

impl CommandOption for UserId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::User(user) => Some(user),
            _ => None,
        }
    }
}

impl CommandOption for ChannelId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Channel;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Channel(channel) => Some(channel),
            _ => None,
        }
    }
}

impl CommandOption for RoleId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Role;

    fn from_option(
        value: CommandOptionValue,
        _: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Role(role) => Some(role),
            _ => None,
        }
    }
}

macro_rules! lookup {
    ($resolved:ident.$cat:ident[$id:expr]) => {
        $resolved.and_then(|resolved| resolved.$cat.iter().find(|val| val.id == $id).cloned())
    };
}

impl CommandOption for User {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::User(user) => lookup!(resolved.users[user]),
            _ => None,
        }
    }
}

impl CommandOption for InteractionMember {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::User(member) => lookup!(resolved.members[member]),
            _ => None,
        }
    }
}

impl CommandOption for (User, InteractionMember) {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::User(user) => {
                lookup!(resolved.members[user]).map(|m| (lookup!(resolved.users[user]).unwrap(), m))
            }
            _ => None,
        }
    }
}

impl CommandOption for (User, Option<InteractionMember>) {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::User(user) => {
                lookup!(resolved.users[user]).map(|u| (u, lookup!(resolved.members[user])))
            }
            _ => None,
        }
    }
}

impl CommandOption for InteractionChannel {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Channel;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Channel(channel) => lookup!(resolved.channels[channel]),
            _ => None,
        }
    }
}

impl CommandOption for Role {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Role;

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        match value {
            CommandOptionValue::Role(role) => lookup!(resolved.roles[role]),
            _ => None,
        }
    }
}

impl<T: CommandOption> CommandOption for Option<T> {
    const OPTION_TYPE: CommandOptionType = T::OPTION_TYPE;
    const DEFAULT: Option<Self> = Some(None);

    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Option<Self> {
        Some(T::from_option(value, resolved))
    }
}

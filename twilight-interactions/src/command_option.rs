use std::str::FromStr;

use twilight_model::{
    application::{
        command::CommandOptionType,
        interaction::application_command::{
            CommandDataOption, CommandInteractionDataResolved, InteractionChannel,
            InteractionMember,
        },
    },
    guild::Role,
    id::{ChannelId, GenericId, RoleId, UserId},
    user::User,
};

use crate::error::ParseErrorType;

/// Trait to convert a slash command option into a concrete type.
///
/// This trait is implemented for primitive Rust types, ID types and some
/// concrete Discord types that can be resolved from a command option.
///
/// The `from_option` method should only fails if an invalid option is provided,
/// and not when its a user error. For example, this trait is implemented for
/// [`Option<InteractionMember>`] but not for [`InteractionMember`] as there is
/// not guarantee that member data will be present when receiving a `USER`
/// option.
pub trait CommandOption: Sized {
    /// Expected option type.
    ///
    /// This value is used when generating slash command builder.
    const OPTION_TYPE: CommandOptionType;

    /// Convert a [`CommandDataOption`] into this value.
    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType>;
}

/// A resolved Discord user.
///
/// This struct implement [`CommandOption`] and can be used to
/// obtain resolved data for a given user id. The struct holds
/// a [`User`] and maybe an [`InteractionMember`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedUser {
    /// The resolved user.
    pub resolved: User,
    /// The resolved member, if found.
    pub member: Option<InteractionMember>,
}

fn parse_id<T: From<u64>>(value: &str) -> Result<T, ParseErrorType> {
    match u64::from_str(value) {
        Ok(id) => Ok(id.into()),
        Err(_) => Err(ParseErrorType::ParseId(value.to_string())),
    }
}

macro_rules! lookup {
    ($resolved:ident.$cat:ident, $id:expr) => {
        $resolved
            .and_then(|resolved| resolved.$cat.iter().find(|val| val.id == $id).cloned())
            .ok_or_else(|| ParseErrorType::LookupFailed($id.0))
    };
}

impl CommandOption for String {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::String;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::String { value, .. } => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for i64 {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Integer;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::Integer { value, .. } => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for bool {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Boolean;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::Boolean { value, .. } => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for UserId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::String { value, .. } => parse_id(&value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for ChannelId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Channel;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::String { value, .. } => parse_id(&value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for RoleId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Role;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::String { value, .. } => parse_id(&value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for GenericId {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Mentionable;

    fn from_option(
        value: CommandDataOption,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandDataOption::String { value, .. } => parse_id(&value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for User {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let user_id: UserId = match value {
            CommandDataOption::String { value, .. } => parse_id(&value)?,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.users, user_id)
    }
}

impl CommandOption for Option<InteractionMember> {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let user_id: UserId = match value {
            CommandDataOption::String { value, .. } => parse_id(&value)?,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        Ok(lookup!(resolved.members, user_id).ok())
    }
}

impl CommandOption for ResolvedUser {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::User;

    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let user_id: UserId = match value {
            CommandDataOption::String { value, .. } => parse_id(&value)?,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        Ok(Self {
            resolved: lookup!(resolved.users, user_id)?,
            member: lookup!(resolved.members, user_id).ok(),
        })
    }
}

impl CommandOption for InteractionChannel {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Channel;

    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let channel_id: ChannelId = match value {
            CommandDataOption::String { value, .. } => parse_id(&value)?,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.channels, channel_id)
    }
}

impl CommandOption for Role {
    const OPTION_TYPE: CommandOptionType = CommandOptionType::Role;

    fn from_option(
        value: CommandDataOption,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let role_id: RoleId = match value {
            CommandDataOption::String { value, .. } => parse_id(&value)?,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.roles, role_id)
    }
}

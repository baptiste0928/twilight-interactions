use twilight_model::{
    application::interaction::application_command::{
        CommandInteractionDataResolved, CommandOptionValue, InteractionChannel, InteractionMember,
    },
    guild::Role,
    id::{ChannelId, GenericId, RoleId, UserId},
    user::User,
};

use crate::error::ParseErrorType;

/// Trait to convert a [`CommandOptionValue`] into a concrete type.
///
/// ## Provided implementations
///
/// | Option type         | Implemented types                      |
/// |---------------------|----------------------------------------|
/// | `STRING`            | [`String`]                             |
/// | `INTEGER`           | [`i64`]                                |
/// | `BOOLEAN`           | [`bool`]                               |
/// | `USER`              | [`ResolvedUser`], [`User`], [`UserId`] |
/// | `CHANNEL`           | [`InteractionChannel`], [`ChannelId`]  |
/// | `ROLE`              | [`Role`], [`RoleId`]                   |
/// | `MENTIONABLE`       | [`GenericId`]                          |
/// | `SUB_COMMAND`       | Not yet implemented.                   |
/// | `SUB_COMMAND_GROUP` | Not yet implemented.                   |
///
/// This trait is only implemented for types where the conversion cannot
/// fail due to a user error (when input is considered as invalid by your
/// application but is valid according the requested data type).
/// For example, this is why the trait is only implemented for [`i64`].
pub trait CommandOption: Sized {
    /// Convert a [`CommandOptionValue`] into this value.
    fn from_option(
        value: CommandOptionValue,
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

macro_rules! lookup {
    ($resolved:ident.$cat:ident, $id:expr) => {
        $resolved
            .and_then(|resolved| resolved.$cat.iter().find(|val| val.id == $id).cloned())
            .ok_or_else(|| ParseErrorType::LookupFailed($id.0))
    };
}

impl CommandOption for String {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::String(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for i64 {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::Integer(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for bool {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::Boolean(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for UserId {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::User(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for ChannelId {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::Channel(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for RoleId {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::Role(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for GenericId {
    fn from_option(
        value: CommandOptionValue,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        match value {
            CommandOptionValue::Mentionable(value) => Ok(value),
            other => Err(ParseErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for User {
    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let user_id: UserId = match value {
            CommandOptionValue::User(value) => value,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.users, user_id)
    }
}

impl CommandOption for ResolvedUser {
    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let user_id: UserId = match value {
            CommandOptionValue::User(value) => value,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        Ok(Self {
            resolved: lookup!(resolved.users, user_id)?,
            member: lookup!(resolved.members, user_id).ok(),
        })
    }
}

impl CommandOption for InteractionChannel {
    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let channel_id: ChannelId = match value {
            CommandOptionValue::Channel(value) => value,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.channels, channel_id)
    }
}

impl CommandOption for Role {
    fn from_option(
        value: CommandOptionValue,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseErrorType> {
        let role_id: RoleId = match value {
            CommandOptionValue::Role(value) => value,
            other => return Err(ParseErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.roles, role_id)
    }
}

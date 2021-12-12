use std::borrow::Cow;

use twilight_model::{
    application::{
        command::{CommandOptionValue as NumberCommandOptionValue, Number},
        interaction::application_command::{
            CommandData, CommandDataOption, CommandInteractionDataResolved, CommandOptionValue,
            InteractionChannel, InteractionMember,
        },
    },
    guild::Role,
    id::{ChannelId, GenericId, RoleId, UserId},
    user::User,
};

use crate::error::{ParseError, ParseOptionErrorType};

use super::internal::CommandOptionData;

/// Parse command data into a concrete type.
///
/// This trait represent a slash command model, that can be initialized
/// from a [`CommandInputData`]. See the module-level documentation to learn more.
///
/// ## Derive macro
/// A derive macro is provided to implement this trait. The macro only works
/// with structs with named fields where all field types implement [`CommandOption`].
///
/// ### Macro attributes
/// The macro provide a `#[command]` attribute to configure generated code.
///
/// **Type parameters**:
/// - `#[command(partial = true)]`: set the model as partial.[^partial]
///
/// **Field parameters**:
/// - `#[command(rename = "")]`: use a different name for the field when parsing.
///
/// ## Example
/// ```
/// use twilight_interactions::command::{CommandModel, ResolvedUser};
///
/// #[derive(CommandModel)]
/// struct HelloCommand {
///     message: String,
///     user: Option<ResolvedUser>
/// }
/// ```
///
/// [^partial]: Unknown fields don't fail the parsing. Useful for parsing autocomplete
///             interaction data.
pub trait CommandModel: Sized {
    /// Construct this type from a [`CommandInputData`].
    fn from_interaction(data: CommandInputData) -> Result<Self, ParseError>;
}

/// Convert a [`CommandOptionValue`] into a concrete type.
///
/// This trait is used by the implementation of [`CommandData`] generated
/// by the derive macro.
///
/// ## Derive macro
/// A derive macro is provided to implement this trait for slash command
/// options with predefined choices. The macro only works on enums and
/// require that the `#[option]` attribute (see below) is present on
/// each variant.
///
/// ### Macro attributes
/// The macro provide a `#[option]` attribute to configure the generated code.
///
/// ***Variant parameters:**
/// - `#[option(name = "")]`: name of the command option choice
/// - `#[option(value = ..)]`: value of the command option choice (either string, integer or float)
///
/// ## Example
/// ```
/// use twilight_interactions::command::CommandOption;
///
/// #[derive(CommandOption)]
/// enum TimeUnit {
///     #[option(name = "Minute", value = 60)]
///     Minute,
///     #[option(name = "Hour", value = 3600)]
///     Hour,
///     #[option(name = "Day", value = 86400)]
///     Day
/// }
/// ```
pub trait CommandOption: Sized {
    /// Convert a [`CommandOptionValue`] into this value.
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType>;
}

/// Data sent by Discord when receiving a command.
///
/// This type is used in the [`CommandModel`] trait. It can be initialized
/// from a [`CommandData`] using the [`From`] trait.
///
/// [`CommandModel`]: super::CommandModel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInputData<'a> {
    pub options: Vec<CommandDataOption>,
    pub resolved: Option<Cow<'a, CommandInteractionDataResolved>>,
}

impl<'a> CommandInputData<'a> {
    /// Parse a subcommand [`CommandOptionValue`].
    ///
    /// This method signature is the same as the [`CommandOption`] trait,
    /// except the explicit `'a` lifetime. It is used when parsing subcommands.
    pub fn from_option(
        value: CommandOptionValue,
        resolved: Option<&'a CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let options = match value {
            CommandOptionValue::SubCommand(options)
            | CommandOptionValue::SubCommandGroup(options) => options,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        Ok(CommandInputData {
            options,
            resolved: resolved.map(Cow::Borrowed),
        })
    }
}

impl From<CommandData> for CommandInputData<'_> {
    fn from(data: CommandData) -> Self {
        Self {
            options: data.options,
            resolved: data.resolved.map(Cow::Owned),
        }
    }
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
            .and_then(|resolved| resolved.$cat.get(&$id).cloned())
            .ok_or_else(|| ParseOptionErrorType::LookupFailed($id.0))
    };
}

impl CommandOption for String {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::String(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for i64 {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let value = match value {
            CommandOptionValue::Integer(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if let Some(NumberCommandOptionValue::Integer(min)) = data.min_value {
            if value < min {
                return Err(ParseOptionErrorType::IntegerOutOfRange(value));
            }
        }

        if let Some(NumberCommandOptionValue::Integer(max)) = data.max_value {
            if value > max {
                return Err(ParseOptionErrorType::IntegerOutOfRange(value));
            }
        }

        Ok(value)
    }
}

impl CommandOption for Number {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let value = match value {
            CommandOptionValue::Number(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if let Some(NumberCommandOptionValue::Number(min)) = data.min_value {
            if value.0 < min.0 {
                return Err(ParseOptionErrorType::NumberOutOfRange(value));
            }
        }

        if let Some(NumberCommandOptionValue::Number(max)) = data.max_value {
            if value.0 > max.0 {
                return Err(ParseOptionErrorType::NumberOutOfRange(value));
            }
        }

        Ok(value)
    }
}

impl CommandOption for f64 {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        Number::from_option(value, data, resolved).map(|val| val.0)
    }
}

impl CommandOption for bool {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Boolean(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for UserId {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::User(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for ChannelId {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Channel(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for RoleId {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Role(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for GenericId {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Mentionable(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for User {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let user_id: UserId = match value {
            CommandOptionValue::User(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.users, user_id)
    }
}

impl CommandOption for ResolvedUser {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let user_id: UserId = match value {
            CommandOptionValue::User(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
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
        data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let resolved = match value {
            CommandOptionValue::Channel(value) => lookup!(resolved.channels, value)?,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if !data.channel_types.is_empty() && !data.channel_types.contains(&resolved.kind) {
            return Err(ParseOptionErrorType::InvalidChannelType(resolved.kind));
        }

        Ok(resolved)
    }
}

impl CommandOption for Role {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let role_id: RoleId = match value {
            CommandOptionValue::Role(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.roles, role_id)
    }
}

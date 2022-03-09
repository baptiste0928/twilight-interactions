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
    id::{
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
        Id,
    },
    user::User,
};

use crate::error::{ParseError, ParseOptionError, ParseOptionErrorType};

use super::internal::CommandOptionData;

/// Parse command data into a concrete type.
///
/// This trait is used to parse received command data into a concrete
/// command model. A derive macro is provided to implement this trait
/// automatically.
///
/// ## Command models
/// This trait can be implemented on structs representing a slash command
/// model. All type fields must implement the [`CommandOption`] trait. A
/// unit struct can be used if the command has no options. See the
/// [module documentation](crate::command) for a full list of supported types.
///
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
/// ### Validating options
/// The [`CommandModel`] trait only focus on parsing received interaction data
/// and does not directly support additional validation. However, it will ensure
/// that received data matches with the provided model. If you specify a `max_value`
/// for a field, this requirement will be checked when parsing command data.
///
/// Not supporting additional validation is a design choice. This allow to clearly
/// split between validations that are ensured by Discord, and those you perform
/// on top of that. If an error occurs during parsing, it is always a bug, not
/// a user mistake.
///
/// If you need to perform additional validation, consider creating another type
/// that can be initialized from the command model.
///
/// ### Autocomplete interactions
/// When receiving an autocomplete interaction, you sometimes only care
/// about a subset of received fields. You can use the
/// `#[command(partial = true)]` attribute to ignore errors related to
/// unknown fields. The [`CreateCommand`] trait cannot be applied on a
/// partial model.
///
/// ## Subcommands and subcommands groups
/// This trait also support parsing subcommands and subcommands group when
/// implemented on enums with all variants containing types that implement
/// [`CommandModel`]. Each variant must have an attribute with the subcommand
/// name.
///
/// Subcommand groups works in the same way as regular subcommands, except the
/// variant type is another enum implementing [`CommandModel`].
///
/// ```
/// use twilight_interactions::command::CommandModel;
/// #
/// # #[derive(CommandModel)]
/// # struct HelloUser {
/// #    message: String,
/// # }
/// #
/// # #[derive(CommandModel)]
/// # struct HelloConfig {
/// #    message: String,
/// # }
///
/// #[derive(CommandModel)]
/// enum HelloCommand {
///     #[command(name = "user")]
///     User(HelloUser),
///     #[command(name = "config")]
///     Config(HelloConfig)
/// }
/// ```
///
///
/// ## Macro attributes
/// The macro provide a `#[command]` attribute to configure generated code.
///
/// | Attribute                | Type           | Location             | Description                                                     |
/// |--------------------------|----------------|----------------------|-----------------------------------------------------------------|
/// | `partial`                | `bool`         | Type                 | Ignore unknown fields when parsing.                             |
/// | `name`                   | `str`          | Variant (subcommand) | Subcommand name (required).                                     |
/// | `rename`                 | `str`          | Field                | Use a different name for the field when parsing.                |
/// | `channel_types`          | `str`          | Field                | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value` | `i64` or `f64` | Field                | Maximum and/or minimum value permitted.                         |
///
/// ### Example
/// ```
/// use twilight_interactions::command::{CommandModel, ResolvedUser};
///
/// #[derive(CommandModel)]
/// #[command(partial = true)]
/// struct HelloCommand {
///     #[command(rename = "text")]
///     message: String,
///     #[command(max_value = 60)]
///     delay: i64
/// }
/// ```
///
/// [^channel_types]: List of [`ChannelType`] names in snake_case separated by spaces
///                   like `guild_text private`.
///
/// [`CreateCommand`]: super::CreateCommand
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CommandModel: Sized {
    /// Construct this type from a [`CommandInputData`].
    fn from_interaction(data: CommandInputData) -> Result<Self, ParseError>;
}

impl CommandModel for Vec<CommandDataOption> {
    fn from_interaction(data: CommandInputData) -> Result<Self, ParseError> {
        Ok(data.options)
    }
}

/// Parse command option into a concrete type.
///
/// This trait is used by the implementation of [`CommandModel`] generated
/// by the derive macro. See the [module documentation](crate::command) for
/// a list of implemented types.
///
/// ## Option choices
/// This trait can be derived on enums to represent command options with
/// predefined choices. The `#[option]` attribute must be present on each
/// variant.
///
/// The corresponding slash command types is automatically inferred from
/// the `value` attribute. In the example below, the inferred type would
/// be `INTEGER`.
///
/// A `value` method is also generated for each variant to obtain the
/// value of the variant. This method is not described in the trait
/// as it is only implemented for option choices.
///
/// ### Example
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
///
/// assert_eq!(TimeUnit::Minute.value(), 60);
/// ```
///
/// ### Macro attributes
/// The macro provide a `#[option]` attribute to configure the generated code.
///
/// | Attribute | Type                  | Location | Description                                |
/// |-----------|-----------------------|----------|--------------------------------------------|
/// | `name`    | `str`                 | Variant  | Set the name of the command option choice. |
/// | `value`   | `str`, `i64` or `f64` | Variant  | Value of the command option choice.        |
///
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
/// from a [`CommandData`] using the [From] trait.
///
/// [`CommandModel`]: super::CommandModel
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInputData<'a> {
    pub options: Vec<CommandDataOption>,
    pub resolved: Option<Cow<'a, CommandInteractionDataResolved>>,
}

impl<'a> CommandInputData<'a> {
    /// Parse a field from the command data.
    ///
    /// This method can be used to manually parse a field from
    /// raw data, for example with guild custom commands. The
    /// method return [`None`] if the field is not present instead
    /// of returning an error.
    ///
    /// ### Example
    /// ```
    /// use twilight_interactions::command::CommandInputData;
    /// # use twilight_model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};
    /// #
    /// # let options = vec![CommandDataOption { name: "message".into(), value: CommandOptionValue::String("Hello world".into()), focused: false }];
    ///
    /// // `options` is a Vec<CommandDataOption>
    /// let data = CommandInputData { options, resolved: None };
    /// let message = data.parse_field::<String>("message").unwrap();
    ///
    /// assert_eq!(message, Some("Hello world".to_string()));
    /// ```
    pub fn parse_field<T>(&self, name: &str) -> Result<Option<T>, ParseError>
    where
        T: CommandOption,
    {
        // Find command option value
        let value = match self
            .options
            .iter()
            .find(|option| option.name == name)
            .map(|option| &option.value)
        {
            Some(value) => value.clone(),
            None => return Ok(None),
        };

        // Parse command value
        match CommandOption::from_option(
            value,
            CommandOptionData::default(),
            self.resolved.as_deref(),
        ) {
            Ok(value) => Ok(Some(value)),
            Err(kind) => Err(ParseError::Option(ParseOptionError {
                field: name.to_string(),
                kind,
            })),
        }
    }

    /// Get the name of the focused field.
    ///
    /// This method is useful when parsing commands with multiple
    /// autocomplete fields.
    ///
    /// ### Example
    /// ```
    /// use twilight_interactions::command::CommandInputData;
    /// # use twilight_model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};
    /// #
    /// # let options = vec![CommandDataOption { name: "message".into(), value: CommandOptionValue::String("Hello world".into()), focused: true }];
    ///
    /// // `options` is a Vec<CommandDataOption>
    /// let data = CommandInputData { options, resolved: None };
    ///
    /// assert_eq!(data.focused(), Some("message"));
    /// ```
    pub fn focused(&self) -> Option<&str> {
        self.options
            .iter()
            .find(|option| option.focused)
            .map(|option| &*option.name)
    }

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
            .ok_or_else(|| ParseOptionErrorType::LookupFailed($id.get()))
    };
}

impl CommandOption for CommandOptionValue {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        Ok(value)
    }
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

impl CommandOption for Id<UserMarker> {
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

impl CommandOption for Id<ChannelMarker> {
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

impl CommandOption for Id<RoleMarker> {
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

impl CommandOption for Id<GenericMarker> {
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

impl CommandOption for Id<AttachmentMarker> {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&CommandInteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Attachment(value) => Ok(value),
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
        let user_id = match value {
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
        let user_id = match value {
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
        let role_id = match value {
            CommandOptionValue::Role(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.roles, role_id)
    }
}

use std::borrow::Cow;

use twilight_model::{
    application::{
        command::CommandOptionValue as NumberCommandOptionValue,
        interaction::{
            application_command::{CommandData, CommandDataOption, CommandOptionValue},
            InteractionChannel, InteractionDataResolved, InteractionMember,
        },
    },
    channel::Attachment,
    guild::Role,
    id::{
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
        Id,
    },
    user::User,
};

use super::internal::CommandOptionData;
use crate::error::{ParseError, ParseOptionError, ParseOptionErrorType};

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
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// ### Validating options
/// The [`CommandModel`] trait only focuses on parsing received interaction data
/// and does not directly support additional validation. However, it will ensure
/// that received data matches with the provided model. If you specify a
/// `max_value` for a field, this requirement will be checked when parsing
/// command data.
///
/// Not supporting additional validation is a design choice. This allows
/// splitting validations that are ensured by Discord and those you perform
/// on top of them. If an error occurs during parsing, it is always a bug, not
/// a user mistake.
///
/// If you need to perform additional validation, consider creating another type
/// that can be initialized from the command model.
///
/// ### Autocomplete interactions
/// Autocomplete interactions are supported with the `#[command(autocomplete = true)]`
/// attribute. Only autocomplete command models are able to use the [`AutocompleteValue`]
/// type in command fields.
///
/// Since autocomplete interactions are partial interactions, models must meet
/// the following requirements:
/// - Every field should be an [`Option<T>`] or [`AutocompleteValue<T>`], since
///   there is no guarantee that a specific field has been filled before the
///   interaction is submitted.
/// - If a field has autocomplete enabled, its type must be [`AutocompleteValue`]
///   or the parsing will fail, since focused fields are sent as [`String`].
/// - Autocomplete models are **partial**, which means that unknown fields
///   will not make the parsing fail.
/// - It is not possible to derive [`CreateCommand`] on autocomplete models.
///
/// <div class="warning">
///
/// Autocomplete models are not meant to be used alone: you should use a regular
/// model to handle interactions submit, and another for autocomplete interactions.
///
/// </div>
///
/// ```
/// use twilight_interactions::command::{AutocompleteValue, CommandModel, ResolvedUser};
///
/// #[derive(CommandModel)]
/// #[command(autocomplete = true)]
/// struct HelloCommand {
///     message: AutocompleteValue<String>,
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// ## Subcommands and subcommands groups
/// This trait also supports parsing subcommands and subcommand groups when
/// implemented on enums with all variants containing types that implement
/// [`CommandModel`]. Each variant must have an attribute with the subcommand
/// name.
///
/// Subcommand groups work the same way as regular subcommands, except the
/// variant type is another enum implementing [`CommandModel`].
///
/// <div class="warning">
///
/// When using subcommands, you should parse and create the command using the
/// top-level command. See the [`xkcd-bot` example] for example usage.
///
/// [`xkcd-bot` example]: https://github.com/baptiste0928/twilight-interactions/tree/main/examples/xkcd-bot
///
/// </div>
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
///     Config(HelloConfig),
/// }
/// ```
///
///
/// ## Macro attributes
/// The macro provides a `#[command]` attribute to configure generated code.
///
/// | Attribute                  | Type           | Location             | Description                                                     |
/// |----------------------------|----------------|----------------------|-----------------------------------------------------------------|
/// | `name`                     | `str`          | Variant (subcommand) | Subcommand name (required).                                     |
/// | `rename`                   | `str`          | Field                | Use a different name for the field when parsing.                |
/// | `channel_types`            | `str`          | Field                | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value`   | `i64` or `f64` | Field                | Maximum and/or minimum value permitted.                         |
/// | `max_length`, `min_length` | `u16`          | Field                | Maximum and/or minimum string length permitted.                 |
///
/// ### Example
/// ```
/// use twilight_interactions::command::CommandModel;
///
/// #[derive(CommandModel)]
/// struct HelloCommand {
///     #[command(rename = "text")]
///     message: String,
///     #[command(max_value = 60)]
///     delay: i64,
/// }
/// ```
///
/// [^channel_types]: List of [`ChannelType`] names in snake_case separated by spaces
///                   like `guild_text private`.
///
/// [`CreateCommand`]: super::CreateCommand
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CommandModel: Sized {
    /// Construct this type from [`CommandInputData`].
    fn from_interaction(data: CommandInputData) -> Result<Self, ParseError>;
}

impl<T: CommandModel> CommandModel for Box<T> {
    fn from_interaction(data: CommandInputData) -> Result<Self, ParseError> {
        T::from_interaction(data).map(Box::new)
    }
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
/// The corresponding slash command types are automatically inferred from
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
///     Day,
/// }
///
/// assert_eq!(TimeUnit::Minute.value(), 60);
/// ```
///
/// ### Macro attributes
/// The macro provides an `#[option]` attribute to configure the generated code.
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
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType>;
}

/// Data sent by Discord when receiving a command.
///
/// This type is used in the [`CommandModel`] trait. It can be initialized
/// from [`CommandData`] using the [From] trait.
///
/// [`CommandModel`]: super::CommandModel
#[derive(Debug, Clone, PartialEq)]
pub struct CommandInputData<'a> {
    pub options: Vec<CommandDataOption>,
    pub resolved: Option<Cow<'a, InteractionDataResolved>>,
}

impl<'a> CommandInputData<'a> {
    /// Parse a field from the command data.
    ///
    /// This method can be used to manually parse a field from
    /// raw data, for example with guild custom commands. The
    /// method returns [`None`] if the field is not present instead
    /// of returning an error.
    ///
    /// ### Example
    /// ```
    /// use twilight_interactions::command::CommandInputData;
    /// # use twilight_model::application::interaction::application_command::{CommandDataOption, CommandOptionValue};
    /// #
    /// # let options = vec![CommandDataOption { name: "message".into(), value: CommandOptionValue::String("Hello world".into()) }];
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
    /// # use twilight_model::application::{
    /// #   interaction::application_command::{CommandDataOption, CommandOptionValue},
    /// #   command::CommandOptionType,
    /// # };
    /// #
    /// # let options = vec![CommandDataOption { name: "message".into(), value: CommandOptionValue::Focused("Hello world".into(), CommandOptionType::String) }];
    ///
    /// // `options` is a Vec<CommandDataOption>
    /// let data = CommandInputData { options, resolved: None };
    ///
    /// assert_eq!(data.focused(), Some("message"));
    /// ```
    pub fn focused(&self) -> Option<&str> {
        self.options
            .iter()
            .find(|option| matches!(option.value, CommandOptionValue::Focused(_, _)))
            .map(|option| &*option.name)
    }

    /// Parse a subcommand's [`CommandOptionValue`].
    ///
    /// This method's signature is the same as the [`CommandOption`] trait,
    /// except for the explicit `'a` lifetime. It is used when parsing
    /// subcommands.
    pub fn from_option(
        value: CommandOptionValue,
        resolved: Option<&'a InteractionDataResolved>,
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
/// This struct implements [`CommandOption`] and can be used to
/// obtain resolved data for a given user ID. The struct holds
/// a [`User`] and maybe an [`InteractionMember`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedUser {
    /// The resolved user.
    pub resolved: User,
    /// The resolved member, if found.
    pub member: Option<InteractionMember>,
}

/// A resolved mentionable.
///
/// This struct implements [`CommandOption`] and can be used to obtain the
/// resolved data from a mentionable ID, that can be either a user or a role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedMentionable {
    /// User mention.
    User(ResolvedUser),
    /// Role mention.
    Role(Role),
}

impl ResolvedMentionable {
    /// Get the ID of the mentionable.
    pub fn id(&self) -> Id<GenericMarker> {
        match self {
            ResolvedMentionable::User(user) => user.resolved.id.cast(),
            ResolvedMentionable::Role(role) => role.id.cast(),
        }
    }
}

/// An autocomplete command field.
///
/// This type represent a value parsed from an autocomplete field. See "Autocomplete interactions"
/// in [`CommandModel` documentation] for more information.
///
/// [`CommandModel` documentation]: CommandModel
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutocompleteValue<T> {
    /// The field has not been completed yet.
    None,
    /// The field is focused by the user and being completed.
    Focused(String),
    /// The field has been completed by the user.
    Completed(T),
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
        _resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        Ok(value)
    }
}

impl<T> CommandOption for AutocompleteValue<T>
where
    T: CommandOption,
{
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Focused(value, _) => Ok(Self::Focused(value)),
            other => {
                let parsed = T::from_option(other, data, resolved)?;

                Ok(Self::Completed(parsed))
            }
        }
    }
}

impl CommandOption for String {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        _resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let value = match value {
            CommandOptionValue::String(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if let Some(min) = data.min_length {
            if value.len() < usize::from(min) {
                todo!()
            }
        }

        if let Some(max) = data.max_length {
            if value.len() > usize::from(max) {
                todo!()
            }
        }

        Ok(value)
    }
}

impl CommandOption for Cow<'_, str> {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        String::from_option(value, data, resolved).map(Cow::Owned)
    }
}

impl CommandOption for i64 {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        _resolved: Option<&InteractionDataResolved>,
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

impl CommandOption for f64 {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        _resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let value = match value {
            CommandOptionValue::Number(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if let Some(NumberCommandOptionValue::Number(min)) = data.min_value {
            if value < min {
                return Err(ParseOptionErrorType::NumberOutOfRange(value));
            }
        }

        if let Some(NumberCommandOptionValue::Number(max)) = data.max_value {
            if value > max {
                return Err(ParseOptionErrorType::NumberOutOfRange(value));
            }
        }

        Ok(value)
    }
}

impl CommandOption for bool {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        _resolved: Option<&InteractionDataResolved>,
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
        _resolved: Option<&InteractionDataResolved>,
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
        _resolved: Option<&InteractionDataResolved>,
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
        _resolved: Option<&InteractionDataResolved>,
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
        _resolved: Option<&InteractionDataResolved>,
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
        _resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        match value {
            CommandOptionValue::Attachment(value) => Ok(value),
            other => Err(ParseOptionErrorType::InvalidType(other.kind())),
        }
    }
}

impl CommandOption for Attachment {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let attachment_id = match value {
            CommandOptionValue::Attachment(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.attachments, attachment_id)
    }
}

impl CommandOption for User {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
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
        resolved: Option<&InteractionDataResolved>,
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

impl CommandOption for ResolvedMentionable {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let id = match value {
            CommandOptionValue::Mentionable(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        let user_id = id.cast();
        if let Ok(user) = lookup!(resolved.users, user_id) {
            let resolved_user = ResolvedUser {
                resolved: user,
                member: lookup!(resolved.members, user_id).ok(),
            };

            return Ok(Self::User(resolved_user));
        }

        let role_id = id.cast();
        if let Ok(role) = lookup!(resolved.roles, role_id) {
            return Ok(Self::Role(role));
        }

        Err(ParseOptionErrorType::LookupFailed(id.into()))
    }
}

impl CommandOption for InteractionChannel {
    fn from_option(
        value: CommandOptionValue,
        data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let resolved = match value {
            CommandOptionValue::Channel(value) => lookup!(resolved.channels, value)?,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        if let Some(channel_types) = data.channel_types {
            if !channel_types.contains(&resolved.kind) {
                return Err(ParseOptionErrorType::InvalidChannelType(resolved.kind));
            }
        }

        Ok(resolved)
    }
}

impl CommandOption for Role {
    fn from_option(
        value: CommandOptionValue,
        _data: CommandOptionData,
        resolved: Option<&InteractionDataResolved>,
    ) -> Result<Self, ParseOptionErrorType> {
        let role_id = match value {
            CommandOptionValue::Role(value) => value,
            other => return Err(ParseOptionErrorType::InvalidType(other.kind())),
        };

        lookup!(resolved.roles, role_id)
    }
}

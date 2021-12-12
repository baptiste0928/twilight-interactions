use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType, Number, OptionsCommandOptionData},
        interaction::application_command::InteractionChannel,
    },
    guild::Role,
    id::{ChannelId, CommandVersionId, GenericId, RoleId, UserId},
    user::User,
};

use super::{internal::CreateOptionData, ResolvedUser};

/// Create a slash command from a type.
///
/// This trait is used to create commands from command models. A derive
/// macro is provided to automatically implement the traits.
///
/// ## Types and fields documentation
/// The trait can be derived structs where all fields implement [`CreateOption`]
/// (see the [module documentation](crate::command) for a list of supported types)
/// or enums where variants implements [`CreateCommand`].
///
/// Unlike the [`CommandModel`] trait, the type its field or variants must have
/// a description. The description correspond either to the first line of the
/// documentation comment, or the value of the `desc` attribute. The type must
/// also be named witht he `name` attribute.
///
/// ## Example
/// ```
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(name = "hello", desc = "Say hello")]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// ## Macro attributes
/// The macro provide a `#[command]` attribute to provide additional information.
///
/// | Attribute                | Type           | Location               | Description                                                     |
/// |--------------------------|----------------|------------------------|-----------------------------------------------------------------|
/// | `name`                   | `str`          | Type                   | Name of the command (required).                                 |
/// | `desc`                   | `str`          | Type / Field / Variant | Set the subcommand name (required).                             |
/// | `default_permission`     | `bool`         | Type                   | Whether the command should be enabled by default.               |
/// | `rename`                 | `str`          | Field                  | Use a different option name than the field name.                |
/// | `autocomplete`           | `bool`         | Field                  | Enable autocomplete on this field.                              |
/// | `channel_types`          | `str`          | Field                  | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value` | `i64` or `f64` | Field                  | Set the maximum and/or minimum value permitted.                 |
///
/// ## Example
/// ```
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(name = "hello", desc = "Say hello")]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
/// ```
///
/// [^channel_types]: List [`ChannelType`] names in snake_case separated by spaces
///                   like `guild_text private`.
///
/// [`CommandModel`]: super::CommandModel
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CreateCommand: Sized {
    /// Create an [`ApplicationCommandData`] for this type.
    fn create_command() -> ApplicationCommandData;
}

/// Create a command option from a type.
///
/// This trait is used by the implementation of [`CreateCommand`] generated
/// by the derive macro. See the [module documentation](crate::command) for
/// a list of implemented types.
///
///
/// ## Option choices
/// This trait can be derived on enums to represent command options with
/// predefined choices. The `#[option]` attribute must be present on each
/// variant.
///
/// ### Example
/// ```
/// use twilight_interactions::command::CreateOption;
///
/// #[derive(CreateOption)]
/// enum TimeUnit {
///     #[option(name = "Minute", value = 60)]
///     Minute,
///     #[option(name = "Hour", value = 3600)]
///     Hour,
///     #[option(name = "Day", value = 86400)]
///     Day
/// }
/// ```
///
/// ### Macro attributes
/// The macro provide a `#[option]` attribute to configure the generated code.
///
/// | Attribute | Type                  | Location | Description                                |
/// |-----------|-----------------------|----------|--------------------------------------------|
/// | `name`    | `str`                 | Variant  | Set the name of the command option choice. |
/// | `value`   | `str`, `i64` or `f64` | Variant  | Value of the command option choice.        |

pub trait CreateOption: Sized {
    /// Create a [`CommandOption`] from this type.
    fn create_option(data: CreateOptionData) -> CommandOption;
}

/// Data sent to discord to create a command.
///
/// This type is used in the [`CreateCommand`] trait.
/// To convert it into a [`Command`], use the [From] (or [Into]) trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// List of command options.
    pub options: Vec<CommandOption>,
    /// Whether the command is enabled by default when the app is added to a guild.
    pub default_permission: bool,
    /// Whether the command is a subcommand group.
    pub group: bool,
}

impl From<ApplicationCommandData> for Command {
    fn from(item: ApplicationCommandData) -> Self {
        Command {
            application_id: None,
            guild_id: None,
            name: item.name,
            default_permission: Some(item.default_permission),
            description: item.description,
            id: None,
            kind: CommandType::ChatInput,
            options: item.options,
            version: CommandVersionId::new(1).unwrap(),
        }
    }
}

impl From<ApplicationCommandData> for CommandOption {
    fn from(item: ApplicationCommandData) -> Self {
        let data = OptionsCommandOptionData {
            description: item.description,
            name: item.name,
            options: item.options,
        };

        if item.group {
            CommandOption::SubCommandGroup(data)
        } else {
            CommandOption::SubCommand(data)
        }
    }
}

impl CreateOption for String {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::String(data.into_choice(Vec::new()))
    }
}

impl CreateOption for i64 {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Integer(data.into_number(Vec::new()))
    }
}

impl CreateOption for Number {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Number(data.into_number(Vec::new()))
    }
}

impl CreateOption for f64 {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Number(data.into_number(Vec::new()))
    }
}

impl CreateOption for bool {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Boolean(data.into_data())
    }
}

impl CreateOption for UserId {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for ChannelId {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Channel(data.into_channel())
    }
}

impl CreateOption for RoleId {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Role(data.into_data())
    }
}

impl CreateOption for GenericId {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Mentionable(data.into_data())
    }
}

impl CreateOption for User {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for ResolvedUser {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for InteractionChannel {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Channel(data.into_channel())
    }
}

impl CreateOption for Role {
    fn create_option(data: CreateOptionData) -> CommandOption {
        CommandOption::Role(data.into_data())
    }
}

use std::{borrow::Cow, collections::HashMap};

use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandOptionType, CommandType},
        interaction::InteractionChannel,
    },
    channel::Attachment,
    guild::{Permissions, Role},
    id::{
        marker::{AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker},
        Id,
    },
    user::User,
};

use super::{internal::CreateOptionData, ResolvedMentionable, ResolvedUser};

/// Create a slash command from a type.
///
/// This trait is used to create commands from command models. A derive
/// macro is provided to automatically implement the traits.
///
/// ## Types and fields documentation
/// The trait can be derived on structs whose fields implement [`CreateOption`]
/// (see the [module documentation](crate::command) for a list of supported
/// types) or enums whose variants implement [`CreateCommand`].
///
/// Unlike the [`CommandModel`] trait, all fields or variants of the type it's
/// implemented on must have a description. The description corresponds either
/// to the first line of the documentation comment or the value of the `desc`
/// attribute. The type must also be named with the `name` attribute.
///
/// ## Example
/// ```
/// # use twilight_model::guild::Permissions;
/// use twilight_interactions::command::{CreateCommand, ResolvedUser};
///
/// #[derive(CreateCommand)]
/// #[command(
///     name = "hello",
///     desc = "Say hello",
///     default_permissions = "hello_permissions"
/// )]
/// struct HelloCommand {
///     /// The message to send.
///     message: String,
///     /// The user to send the message to.
///     user: Option<ResolvedUser>,
/// }
///
/// fn hello_permissions() -> Permissions {
///     Permissions::SEND_MESSAGES
/// }
/// ```
///
/// ## Macro attributes
/// The macro provides a `#[command]` attribute to provide additional
/// information.
///
/// | Attribute                  | Type                | Location               | Description                                                     |
/// |----------------------------|---------------------|------------------------|-----------------------------------------------------------------|
/// | `name`                     | `str`               | Type                   | Name of the command (required).                                 |
/// | `desc`                     | `str`               | Type / Field / Variant | Description of the command (required).                          |
/// | `default_permissions`      | `fn`[^perms]        | Type                   | Default permissions required by members to run the command.     |
/// | `dm_permission`            | `bool`              | Type                   | Whether the command can be run in DMs.                          |
/// | `nsfw`                     | `bool`              | Type                   | Whether the command is age-restricted.
/// | `rename`                   | `str`               | Field                  | Use a different option name than the field name.                |
/// | `name_localizations`       | `fn`[^localization] | Type / Field / Variant | Localized name of the command (optional).                       |
/// | `desc_localizations`       | `fn`[^localization] | Type / Field / Variant | Localized description of the command (optional).                |
/// | `autocomplete`             | `bool`              | Field                  | Enable autocomplete on this field.                              |
/// | `channel_types`            | `str`               | Field                  | Restricts the channel choice to specific types.[^channel_types] |
/// | `max_value`, `min_value`   | `i64` or `f64`      | Field                  | Set the maximum and/or minimum value permitted.                 |
/// | `max_length`, `min_length` | `u16`               | Field                  |   Maximum and/or minimum string length permitted.               |
///
/// [^perms]: Path to a function that returns [`Permissions`].
///
/// [^localization]: Path to a function that returns a type that implements
/// `IntoIterator<Item = (ToString, ToString)>`. See the module documentation to
/// learn more.
///
/// [^channel_types]: List of [`ChannelType`] names in snake_case separated by spaces
/// like `guild_text private`.
///
/// [`CommandModel`]: super::CommandModel
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CreateCommand: Sized {
    /// Name of the command.
    const NAME: &'static str;

    /// Create an [`ApplicationCommandData`] for this type.
    fn create_command() -> ApplicationCommandData;
}

impl<T: CreateCommand> CreateCommand for Box<T> {
    const NAME: &'static str = T::NAME;

    fn create_command() -> ApplicationCommandData {
        T::create_command()
    }
}

/// Create a command option from a type.
///
/// This trait is used by the implementation of [`CreateCommand`] generated
/// by the derive macro. See the [module documentation](crate::command) for
/// a list of implemented types.
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
///     Day,
/// }
/// ```
///
/// ### Macro attributes
/// The macro provides an `#[option]` attribute to configure the generated code.
///
/// | Attribute            | Type                  | Location | Description                                  |
/// |----------------------|-----------------------|----------|----------------------------------------------|
/// | `name`               | `str`                 | Variant  | Set the name of the command option choice.   |
/// | `name_localizations` | `fn`[^localization]   | Variant  | Localized name of the command option choice. |
/// | `value`              | `str`, `i64` or `f64` | Variant  | Value of the command option choice.          |
///
/// [^localization]: Path to a function that returns a type that implements
///                  `IntoIterator<Item = (ToString, ToString)>`. See the
///                  [module documentation](crate::command) to learn more.

pub trait CreateOption: Sized {
    /// Create a [`CommandOption`] from this type.
    fn create_option(data: CreateOptionData) -> CommandOption;
}

/// Data sent to Discord to create a command.
///
/// This type is used in the [`CreateCommand`] trait.
/// To convert it into a [`Command`], use the [From] (or [Into]) trait.
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the command name. Keys must be valid
    /// locales.
    pub name_localizations: Option<HashMap<String, String>>,
    /// Description of the command. It must be 100 characters or less.
    pub description: String,
    /// Localization dictionary for the command description. Keys must be valid
    /// locales.
    pub description_localizations: Option<HashMap<String, String>>,
    /// List of command options.
    pub options: Vec<CommandOption>,
    /// Whether the command is available in DMs.
    pub dm_permission: Option<bool>,
    /// Default permissions required for a member to run the command.
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is a subcommand group.
    pub group: bool,
    /// Whether the command is nsfw.
    pub nsfw: Option<bool>,
}

impl From<ApplicationCommandData> for Command {
    fn from(item: ApplicationCommandData) -> Self {
        Command {
            application_id: None,
            guild_id: None,
            name: item.name,
            name_localizations: item.name_localizations,
            default_member_permissions: item.default_member_permissions,
            dm_permission: item.dm_permission,
            description: item.description,
            description_localizations: item.description_localizations,
            id: None,
            kind: CommandType::ChatInput,
            nsfw: item.nsfw,
            options: item.options,
            version: Id::new(1),
        }
    }
}

impl From<ApplicationCommandData> for CommandOption {
    fn from(item: ApplicationCommandData) -> Self {
        let data = CreateOptionData {
            name: item.name,
            name_localizations: item.name_localizations,
            description: item.description,
            description_localizations: item.description_localizations,
            required: None,
            autocomplete: false,
            data: Default::default(),
        };

        if item.group {
            data.builder(CommandOptionType::SubCommandGroup)
                .options(item.options)
                .build()
        } else {
            data.builder(CommandOptionType::SubCommand)
                .options(item.options)
                .build()
        }
    }
}

impl CreateOption for String {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::String)
    }
}

impl<'a> CreateOption for Cow<'a, str> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::String)
    }
}

impl CreateOption for i64 {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Integer)
    }
}

impl CreateOption for f64 {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Number)
    }
}

impl CreateOption for bool {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Boolean)
    }
}

impl CreateOption for Id<UserMarker> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for Id<ChannelMarker> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Channel)
    }
}

impl CreateOption for Id<RoleMarker> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Role)
    }
}

impl CreateOption for Id<GenericMarker> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Mentionable)
    }
}

impl CreateOption for Id<AttachmentMarker> {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Attachment)
    }
}

impl CreateOption for Attachment {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Attachment)
    }
}

impl CreateOption for User {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for ResolvedUser {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::User)
    }
}

impl CreateOption for ResolvedMentionable {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Mentionable)
    }
}

impl CreateOption for InteractionChannel {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Channel)
    }
}

impl CreateOption for Role {
    fn create_option(data: CreateOptionData) -> CommandOption {
        data.into_option(CommandOptionType::Role)
    }
}

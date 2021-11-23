use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, Command,
            CommandOption, CommandOptionChoice, CommandOptionValue, CommandType, Number,
            NumberCommandOptionData, OptionsCommandOptionData,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::ChannelType,
    guild::Role,
    id::{ChannelId, CommandVersionId, GenericId, RoleId, UserId},
    user::User,
};

#[cfg(feature = "http")]
use twilight_http::{request::application::InteractionError, response::ResponseFuture, Client};
#[cfg(feature = "http")]
use twilight_model::id::GuildId;

use super::ResolvedUser;

/// Create a [`ApplicationCommandData`] from a type.
///
/// This trait allow to obtain command information from a type.
/// See the module-level documentation to learn more.
///
/// ## Derive macro
/// A derive macro is provided to implement this trait. The macro only works
/// with structs with named fields where all field types implement [`CreateOption`].
///
/// ### Macro attributes
/// The macro provide a `#[command]` attribute to provide additional information.
///
/// **Type parameters**:
/// - `#[command(name = "")]`: set the command name (*required*).
/// - `#[command(desc = "")]`: set the command description.[^desc]
/// - `#[command(default_permission = true)]`: whether the command should be enabled by default.
///
/// **Field parameters**:
/// - `#[command(rename = "")]`: use a different option name than the field name.
/// - `#[command(desc = "")]`: set the option description.[^desc]
/// - `#[command(autocomplete = true)]`: enable autocomplete on this field.[^autocomplete]
/// - `#[command(channel_types = "")]`: restricts the channel choice to specific types.[^channel_types]
/// - `#[command(max_value = 0, min_value = 0)]`: set the maximum and/or minimum value permitted (integer or float).
///
/// It is mandatory to provide a description for each option and the command itself,
/// either using documentation comments or `desc` attribute parameter.
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
/// [^desc]: Documentation comments can also be used. Only the fist line will be taken in account.
///
/// [^autocomplete]: Parsing of partial data received from autocomplete interaction is not yet supported.
///
/// [^channel_types]: List [`ChannelType`] names in snake_case separated by spaces
///                   like `guild_text private`.
pub trait CreateCommand: Sized {
    /// Create an [`ApplicationCommandData`] for this type.
    fn create_command() -> ApplicationCommandData;
}

/// Data sent to Discord to create a command.
///
/// This type can be converted to a [`Command`] using the
/// [`Into`] trait.
///
/// If the `http` feature is enabled, this type provide
/// two methods to create the command.
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
}

impl ApplicationCommandData {
    #[cfg(feature = "http")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http")))]
    /// Create a global application command from this [`ApplicationCommandData`].
    pub fn create_global_command(
        &self,
        client: &Client,
    ) -> Result<ResponseFuture<Command>, InteractionError> {
        Ok(client
            .create_global_command(&self.name)?
            .chat_input(&self.description)?
            .default_permission(self.default_permission)
            .command_options(&self.options)?
            .exec())
    }

    #[cfg(feature = "http")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http")))]
    /// Create a guild application command from this [`ApplicationCommandData`].
    pub fn create_guild_command(
        &self,
        client: &Client,
        guild_id: GuildId,
    ) -> Result<ResponseFuture<Command>, InteractionError> {
        Ok(client
            .create_guild_command(guild_id, &self.name)?
            .chat_input(&self.description)?
            .default_permission(self.default_permission)
            .command_options(&self.options)?
            .exec())
    }
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

/// Trait to create a [`CommandOption`] from a type.
///
/// This trait allow to create a [`CommandOption`] for a type. It is primarily used in the
/// implementation generated when deriving [`CreateCommand`].
///
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
pub trait CreateOption: Sized {
    /// Create a [`CommandOption`] from this type.
    fn create_option(data: CommandOptionData) -> CommandOption;
}

/// Data of a command option.
///
/// This type is used in the [`CreateOption`] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOptionData {
    /// Name of the option. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// Whether the option is required to be completed by a user.
    pub required: bool,
    /// Whether the command supports autocomplete. Only for `STRING`, `INTEGER` and `NUMBER` option type.
    pub autocomplete: bool,
    /// Restricts the channel choice to specific types. Only for `CHANNEL` option type.
    pub channel_types: Vec<ChannelType>,
    /// Maximum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub max_value: Option<CommandOptionValue>,
    /// Minimum value permitted. Only for `INTEGER` and `NUMBER` option type.
    pub min_value: Option<CommandOptionValue>,
}

impl CommandOptionData {
    /// Conversion into a [`BaseCommandOptionData`]
    pub fn into_data(self) -> BaseCommandOptionData {
        BaseCommandOptionData {
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChannelCommandOptionData`]
    pub fn into_channel(self) -> ChannelCommandOptionData {
        ChannelCommandOptionData {
            channel_types: self.channel_types,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChoiceCommandOptionData`]
    pub fn into_choice(self, choices: Vec<CommandOptionChoice>) -> ChoiceCommandOptionData {
        ChoiceCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`NumberCommandOptionData`]
    pub fn into_number(self, choices: Vec<CommandOptionChoice>) -> NumberCommandOptionData {
        NumberCommandOptionData {
            autocomplete: self.autocomplete,
            choices,
            description: self.description,
            max_value: self.max_value,
            min_value: self.min_value,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`OptionsCommandOptionData`]
    pub fn into_options(self, options: Vec<CommandOption>) -> OptionsCommandOptionData {
        OptionsCommandOptionData {
            description: self.description,
            name: self.name,
            options,
        }
    }
}

impl CreateOption for String {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::String(data.into_choice(Vec::new()))
    }
}

impl CreateOption for i64 {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Integer(data.into_number(Vec::new()))
    }
}

impl CreateOption for Number {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Number(data.into_number(Vec::new()))
    }
}

impl CreateOption for f64 {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Number(data.into_number(Vec::new()))
    }
}

impl CreateOption for bool {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Boolean(data.into_data())
    }
}

impl CreateOption for UserId {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for ChannelId {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Channel(data.into_channel())
    }
}

impl CreateOption for RoleId {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Role(data.into_data())
    }
}

impl CreateOption for GenericId {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Mentionable(data.into_data())
    }
}

impl CreateOption for User {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for ResolvedUser {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::User(data.into_data())
    }
}

impl CreateOption for InteractionChannel {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Channel(data.into_channel())
    }
}

impl CreateOption for Role {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Role(data.into_data())
    }
}

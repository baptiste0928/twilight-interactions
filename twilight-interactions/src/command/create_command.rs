use twilight_model::{
    application::{
        command::{Command, CommandOption, Number},
        interaction::application_command::InteractionChannel,
    },
    guild::Role,
    id::{ChannelId, GenericId, RoleId, UserId},
    user::User,
};

use super::{internal::CommandOptionData, ResolvedUser};

/// Create a [`Command`] from a type.
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
///
/// [`ChannelType`]: twilight_model::channel::ChannelType
pub trait CreateCommand: Sized {
    /// Create an [`Command`] for this type.
    fn create_command() -> Command;
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

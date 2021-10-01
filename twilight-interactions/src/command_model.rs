use twilight_model::application::{
    command::CommandOption as ApplicationCommandOption,
    interaction::application_command::CommandData,
};

use crate::error::ParseError;

/// Trait to parse [`CommandData`] into a concrete type.
///
/// This trait represent a slash command model, that can be initialized from a [`CommandData`].
///
/// ## Derive macro
/// A derive macro is provided to implement this trait. The macro only works on structs with named fields.
///
/// ## Example
/// ```
/// use twilight_interactions::{CommandModel, ResolvedUser};
///
/// #[derive(CommandModel)]
/// struct HelloCommand {
///     message: String,
///     user: Option<ResolvedUser>
/// }
/// ```
pub trait CommandModel: Sized {
    /// Construct this type from a [`CommandData`].
    fn from_interaction(data: CommandData) -> Result<Self, ParseError>;
}

/// Trait to create a [`ApplicationCommandData`] from a type.
pub trait CreateCommand: Sized {
    /// Create an [`ApplicationCommandData`] for this type.
    fn create_command() -> ApplicationCommandData;
}

/// Data sent to Discord to create a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationCommandData {
    /// Name of the command. It must be 32 characters or less.
    pub name: String,
    /// Description of the option. It must be 100 characters or less.
    pub description: String,
    /// List of command options.
    pub options: Vec<ApplicationCommandOption>,
    /// Whether the command is enabled by default when the app is added to a guild.
    pub default_permission: bool,
}

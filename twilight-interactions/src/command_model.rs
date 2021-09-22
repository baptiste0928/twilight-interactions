use twilight_model::application::interaction::application_command::CommandData;

use crate::error::ParseError;

/// Trait to parse slash command data into a concrete type.
///
/// This trait represent a slash command model, that can be initialized from a [`CommandData`].
/// A derive macro is provided to automatically derive this trait.
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

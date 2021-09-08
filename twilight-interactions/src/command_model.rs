use twilight_model::application::interaction::application_command::CommandData;

use crate::error::ParseErrorType;

/// Trait to parse slash command data into a concrete type.
///
/// This trait represent a slash command model, that can be initialized from a [`CommandData`].
pub trait CommandModel: Sized {
    fn from_interaction(data: CommandData) -> Result<Self, ParseErrorType>;
}

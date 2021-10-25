use twilight_model::{
    application::{
        command::{
            BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData,
            CommandOption, CommandOptionChoice, OptionsCommandOptionData,
        },
        interaction::application_command::InteractionChannel,
    },
    channel::ChannelType,
    guild::Role,
    id::{ChannelId, GenericId, RoleId, UserId},
    user::User,
};

use crate::ResolvedUser;

/// Trait to create a [`CommandOption`] from a type.
///
/// This trait allow to create a [`CommandOption`] for a type. It is primarily used in the
/// implementation generated when deriving [`CreateCommand`].
///
/// ## Provided implementations
/// This trait is implemented on the same types as the [`CommandOption`](crate::CommandOption)
/// trait. Please refer to its documentation for the full list.
///
/// [`CreateCommand`]: crate::CreateCommand
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
    pub fn into_channel(self, channel_types: Vec<ChannelType>) -> ChannelCommandOptionData {
        ChannelCommandOptionData {
            channel_types,
            description: self.description,
            name: self.name,
            required: self.required,
        }
    }

    /// Conversion into a [`ChoiceCommandOptionData`]
    pub fn into_choice(self, choices: Vec<CommandOptionChoice>) -> ChoiceCommandOptionData {
        ChoiceCommandOptionData {
            choices,
            description: self.description,
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
        CommandOption::Integer(data.into_choice(Vec::new()))
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
        CommandOption::Channel(data.into_channel(Vec::new()))
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
        CommandOption::Channel(data.into_channel(Vec::new()))
    }
}

impl CreateOption for Role {
    fn create_option(data: CommandOptionData) -> CommandOption {
        CommandOption::Role(data.into_data())
    }
}

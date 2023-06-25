use std::{mem, sync::Arc};

use anyhow::bail;
use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::application::interaction::{
    application_command::CommandData, Interaction, InteractionData,
};

use crate::interactions::command::XkcdCommand;

/// Process incoming interactions from Discord.
pub async fn process_interactions(event: Event, client: Arc<Client>) {
    // We only care about interaction events.
    let mut interaction = match event {
        Event::InteractionCreate(interaction) => interaction.0,
        _ => return,
    };

    // Extract the command data from the interaction.
    // We use mem::take to avoid cloning the data.
    let data = match mem::take(&mut interaction.data) {
        Some(InteractionData::ApplicationCommand(data)) => *data,
        _ => {
            tracing::warn!("ignoring non-command interaction");
            return;
        }
    };

    if let Err(error) = handle_command(interaction, data, &client).await {
        tracing::error!(?error, "error while handling command");
    }
}

/// Handle a command interaction.
async fn handle_command(
    interaction: Interaction,
    data: CommandData,
    client: &Client,
) -> anyhow::Result<()> {
    match &*data.name {
        "xkcd" => XkcdCommand::handle(interaction, data, client).await,
        name => bail!("unknown command: {}", name),
    }
}

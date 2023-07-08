use anyhow::Context;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::{application_command::CommandData, Interaction},
    channel::message::Embed,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{
    embed::{EmbedBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::api::XkcdComic;

/// Explore xkcd comics
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "xkcd")]
pub enum XkcdCommand {
    #[command(name = "latest")]
    Latest(XkcdLatestCommand),
    #[command(name = "number")]
    Number(XkcdNumberCommand),
}

impl XkcdCommand {
    /// Handle incoming `/xkcd` commands.
    pub async fn handle(
        interaction: Interaction,
        data: CommandData,
        client: &Client,
    ) -> anyhow::Result<()> {
        // Parse the command data into a structure using twilight-interactions.
        let command =
            XkcdCommand::from_interaction(data.into()).context("failed to parse command data")?;

        // Call the appropriate subcommand.
        match command {
            XkcdCommand::Latest(command) => command.run(interaction, client).await,
            XkcdCommand::Number(command) => command.run(interaction, client).await,
        }
    }
}

/// Get the latest xkcd comic
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "latest")]
pub struct XkcdLatestCommand;

impl XkcdLatestCommand {
    /// Run the `/xkcd latest` command.
    pub async fn run(&self, interaction: Interaction, client: &Client) -> anyhow::Result<()> {
        let comic = XkcdComic::get_latest().await?;
        let embed = crate_comic_embed(comic)?;

        // Respond to the interaction with an embed.
        let client = client.interaction(interaction.application_id);
        let data = InteractionResponseDataBuilder::new()
            .embeds([embed])
            .build();

        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data),
        };

        client
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }
}

/// Get a specific xkcd comic by number
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "number")]
pub struct XkcdNumberCommand {
    /// Comic number
    #[command(min_value = 1)]
    pub number: i64,
}

impl XkcdNumberCommand {
    /// Run the `/xkcd number <num>` command.
    pub async fn run(&self, interaction: Interaction, client: &Client) -> anyhow::Result<()> {
        let comic = XkcdComic::get_number(self.number.try_into()?).await?;

        let mut data = InteractionResponseDataBuilder::new();
        if let Some(comic) = comic {
            data = data.embeds([crate_comic_embed(comic)?]);
        } else {
            data = data.content(format!("No comic found for number {}", self.number));
        }

        let client = client.interaction(interaction.application_id);
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data.build()),
        };

        client
            .create_response(interaction.id, &interaction.token, &response)
            .await?;

        Ok(())
    }
}

/// Create a Discord embed for a comic
fn crate_comic_embed(comic: XkcdComic) -> anyhow::Result<Embed> {
    let image = ImageSource::url(&comic.image_url)?;
    let title = format!(
        "{}: {} ({}-{}-{})",
        comic.number, comic.title, comic.year, comic.month, comic.day
    );

    let embed = EmbedBuilder::new()
        .color(0x2f3136) // Dark theme color, render a "transparent" background
        .title(title)
        .url(comic.url())
        .image(image)
        .build();

    Ok(embed)
}

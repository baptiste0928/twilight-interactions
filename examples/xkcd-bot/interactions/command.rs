use anyhow::Context;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand, DescLocalizations};
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

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "xkcd", desc_localizations = "xkcd_desc")]
pub enum XkcdCommand {
    #[command(name = "latest")]
    Latest(XkcdLatestCommand),
    #[command(name = "number")]
    Number(XkcdNumberCommand),
    #[command(name = "random")]
    Random(XkcdRandomCommand),
}

fn xkcd_desc() -> DescLocalizations {
    DescLocalizations::new("Explore xkcd comics", [("fr", "Explorer les comics xkcd")])
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
            XkcdCommand::Random(command) => command.run(interaction, client).await,
        }
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "latest", desc_localizations = "xkcd_latest_desc")]
pub struct XkcdLatestCommand;

fn xkcd_latest_desc() -> DescLocalizations {
    DescLocalizations::new(
        "Show the latest xkcd comic",
        [("fr", "Afficher le dernier comic xkcd")],
    )
}

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

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "number", desc_localizations = "xkcd_number_desc")]
pub struct XkcdNumberCommand {
    /// Comic number
    #[command(min_value = 1, desc_localizations = "xkcd_number_arg_desc")]
    pub number: i64,
}

fn xkcd_number_desc() -> DescLocalizations {
    DescLocalizations::new(
        "Show a specific xkcd comic",
        [("fr", "Afficher un comic xkcd spécifique")],
    )
}

fn xkcd_number_arg_desc() -> DescLocalizations {
    DescLocalizations::new("Comic number", [("fr", "Numéro du comic")])
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

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "random", desc_localizations = "xkcd_random_desc")]
pub struct XkcdRandomCommand;

fn xkcd_random_desc() -> DescLocalizations {
    DescLocalizations::new(
        "Show a random xkcd comic",
        [("fr", "Afficher un comic xkcd aléatoire")],
    )
}

impl XkcdRandomCommand {
    /// Run the `/xkcd random` command.
    pub async fn run(&self, interaction: Interaction, client: &Client) -> anyhow::Result<()> {
        let comic = XkcdComic::get_random().await?;
        let embed = crate_comic_embed(comic)?;

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

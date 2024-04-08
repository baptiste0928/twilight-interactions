//! # xkcd example bot
//!
//! This example is a simple bot to explore comics from
//! [xkcd](https://xkcd.com/).

pub mod api;
mod interactions;
mod process;

use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use anyhow::Context;
use tracing::Level;
use twilight_gateway::{
    error::ReceiveMessageErrorType, CloseFrame, ConfigBuilder, Event, EventTypeFlags, Intents,
    Shard, StreamExt as _,
};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{ActivityType, MinimalActivity, Status},
};

use crate::{interactions::command::XkcdCommand, process::process_interactions};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN").context("DISCORD_TOKEN environment variable not set")?;

    // Initialize logging with tracing
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .init();

    // Initialize Twilight HTTP client and gateway configuration.
    let client = Arc::new(Client::new(token.clone()));
    let config = ConfigBuilder::new(token.clone(), Intents::empty())
        .presence(presence())
        .build();

    // Register global commands.
    let commands = [XkcdCommand::create_command().into()];
    let application = client.current_user_application().await?.model().await?;
    let interaction_client = client.interaction(application.id);

    tracing::info!("logged as {} with ID {}", application.name, application.id);

    if let Err(error) = interaction_client.set_global_commands(&commands).await {
        tracing::error!(?error, "failed to register commands");
    }

    // Start gateway shards.
    let shards =
        twilight_gateway::create_recommended(&client, config, |_id, builder| builder.build())
            .await?;
    let shard_len = shards.len();
    let mut senders = Vec::with_capacity(shard_len);
    let mut tasks = Vec::with_capacity(shard_len);

    for shard in shards {
        senders.push(shard.sender());
        tasks.push(tokio::spawn(runner(shard, client.clone())));
    }

    tokio::signal::ctrl_c().await?;
    SHUTDOWN.store(true, Ordering::Relaxed);
    for sender in senders {
        // Ignore error if shard's already shutdown.
        _ = sender.close(CloseFrame::NORMAL);
    }

    for jh in tasks {
        _ = jh.await;
    }

    Ok(())
}

async fn runner(mut shard: Shard, client: Arc<Client>) {
    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let event = match item {
            Ok(Event::GatewayClose(_)) if SHUTDOWN.load(Ordering::Relaxed) => break,
            Ok(event) => event,
            Err(error)
                if SHUTDOWN.load(Ordering::Relaxed)
                    && matches!(error.kind(), ReceiveMessageErrorType::WebSocket) =>
            {
                break
            }
            Err(error) => {
                tracing::warn!(?error, "error while receiving event");
                continue;
            }
        };

        // Process Discord events (see `process.rs` file).
        tracing::info!(kind = ?event.kind(), shard = ?shard.id().number(), "received event");
        tokio::spawn(process_interactions(event, client.clone()));
    }
}

fn presence() -> UpdatePresencePayload {
    let activity = MinimalActivity {
        kind: ActivityType::Watching,
        name: String::from("xkcd comics"),
        url: None,
    };

    UpdatePresencePayload {
        activities: vec![activity.into()],
        afk: false,
        since: None,
        status: Status::Online,
    }
}

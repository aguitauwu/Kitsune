mod bot;
mod config;
mod database;
mod security;
mod utils;

use anyhow::{Context, Result};
use poise::serenity_prelude as serenity;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kitsune=info,serenity=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🦊 Kitsune Guardian Fox - Starting...");

    let config = config::Config::from_env()
        .context("Failed to load configuration")?;

    tracing::info!("Configuration loaded successfully");

    let pool = database::create_pool(&config.database_url)
        .await
        .context("Failed to create database pool")?;

    tracing::info!("Database connection established");

    let redis = utils::redis_client::create_redis_client(&config.redis_url)
        .await
        .context("Failed to connect to Redis")?;

    tracing::info!("Redis connection established");

    let intents = serenity::GatewayIntents::GUILDS
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MODERATION;

    let framework = bot::create_framework(config.clone(), pool.clone(), redis)
        .await
        .context("Failed to create bot framework")?;

    let mut client = serenity::ClientBuilder::new(&config.discord_token, intents)
        .framework(framework)
        .await
        .context("Failed to create Discord client")?;

    tracing::info!("🦊 Kitsune is now online and protecting servers!");

    client.start().await.context("Failed to start client")?;

    Ok(())
}

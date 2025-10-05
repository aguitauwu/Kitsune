use poise::serenity_prelude as serenity;
use super::{Context, Error};

use super::commands::reputation::{reputation_query, reputation_report, reputation_trust, reputation_sync, reputation_servers, reputation_appeal};
use super::commands::whitelist::{whitelist, whitelist_remove, whitelist_list, blacklist, blacklist_remove, blacklist_list, blacklist_import, blacklist_export};
use super::commands::testing::{test, debug, health};
use super::commands::help::{about, docs, invite};
use super::commands::analytics::{analytics, predict, compare};
use super::commands::backup::{backup, backup_restore, backup_list, backup_download, backup_schedule};
use super::commands::lockdown_cmd::lockdown_schedule;
use super::commands::custom::custom;
use super::commands::integration::{webhook, api};

#[poise::command(
    slash_command,
    subcommands(
        "reputation_query", "reputation_report", "reputation_trust", 
        "reputation_sync", "reputation_servers", "reputation_appeal"
    ),
    guild_only = true
)]
pub async fn reputation(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/reputation` subcommands to manage reputation network").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    subcommands(
        "whitelist", "whitelist_remove", "whitelist_list",
        "blacklist", "blacklist_remove", "blacklist_list", "blacklist_import", "blacklist_export"
    ),
    guild_only = true,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn access(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/access` subcommands to manage whitelist and blacklist").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    subcommands("analytics", "predict", "compare"),
    guild_only = true,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn insights(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/insights` subcommands for analytics and predictions").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    subcommands(
        "backup", "backup_restore", "backup_list", 
        "backup_download", "backup_schedule",
        "lockdown_schedule",
        "custom",
        "webhook", "api",
        "test", "debug", "health"
    ),
    guild_only = true,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn admin(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/admin` subcommands for advanced administration").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 Kitsune Guardian Fox - Information")
            .description("**Available Commands:**\n\n• `/kitsune` - Core security & moderation\n• `/reputation` - Reputation network\n• `/access` - Whitelist & blacklist management\n• `/insights` - Analytics & predictions\n• `/admin` - Advanced administration\n• `/info` - This message")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune v1.0 | Guardian Fox"))
    )).await?;
    Ok(())
}

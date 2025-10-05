pub mod security;
pub mod moderation;
pub mod config;
pub mod honeypot;
pub mod stats;
pub mod reputation;
pub mod lockdown_cmd;
pub mod whitelist;
pub mod testing;
pub mod help;
pub mod analytics;
pub mod backup;
pub mod custom;
pub mod integration;

use poise::serenity_prelude as serenity;
use super::{Context, Error};

use security::{status, scan, check, analyze, reputation_cmd};
use moderation::{ban, kick, timeout, warn, unban, pardon};
use config::{view, automod_toggle, channel, notify, raid, behavior, ml};
use honeypot::honeypot;
use stats::{stats, leaderboard, report, forensics, export};
use reputation::{reputation_query, reputation_report, reputation_trust, reputation_sync, reputation_servers, reputation_appeal};
use lockdown_cmd::{lockdown, lockdown_status, lockdown_schedule, verification};
use whitelist::{whitelist, whitelist_remove, whitelist_list, blacklist, blacklist_remove, blacklist_list, blacklist_import, blacklist_export};
use testing::{test, debug, health};
use help::{help, about, docs, invite};
use analytics::{analytics, predict, compare};
use backup::{backup, backup_restore, backup_list, backup_download, backup_schedule};
use custom::custom;
use integration::{webhook, api};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands(
        "status", "scan", "check", "analyze",
        "ban", "kick", "timeout", "warn", "unban", "pardon",
        "view", "automod_toggle", "channel", "notify", "raid", "behavior", "ml",
        "honeypot",
        "stats", "leaderboard", "report", "forensics", "export",
        "lockdown", "verification"
    ),
    guild_only = true
)]
pub async fn kitsune(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 Kitsune Guardian Fox")
            .description("Advanced Discord Security Bot\n\nUse `/help` to see all available commands.\n\n**Quick Commands:**\n• `/kitsune status` - Security status\n• `/kitsune scan` - Full server scan\n• `/kitsune check @user` - Analyze user\n• `/help` - Full command list")
            .color(0x00ff00)
            .footer(serenity::CreateEmbedFooter::new("Kitsune v1.0 | Guardian Fox"))
    )).await?;
    Ok(())
}

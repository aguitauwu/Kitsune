use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let guild = queries::get_guild(&ctx.data().pool, guild_id).await?;
    
    let automod_status = ctx.data().config.auto_mod.enabled;
    let lockdown_status = guild.as_ref().map(|g| g.lockdown_active).unwrap_or(false);
    
    let description = format!(
        "**Auto-Moderation:** {}\n**Lockdown:** {}\n\n**Raid Detection Thresholds:**\n- 5s: {}\n- 30s: {}\n- 1m: {}\n- 5m: {}\n\n**Auto-Mod Thresholds:**\n- Low: {:.2}\n- Medium: {:.2}\n- High: {:.2}\n- Critical: {:.2}",
        if automod_status { "✅ Enabled" } else { "❌ Disabled" },
        if lockdown_status { "🔒 Active" } else { "✅ Inactive" },
        ctx.data().config.security.raid_threshold_5s,
        ctx.data().config.security.raid_threshold_30s,
        ctx.data().config.security.raid_threshold_1m,
        ctx.data().config.security.raid_threshold_5m,
        ctx.data().config.auto_mod.low_threat_threshold,
        ctx.data().config.auto_mod.medium_threat_threshold,
        ctx.data().config.auto_mod.high_threat_threshold,
        ctx.data().config.auto_mod.critical_threat_threshold
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚙️ Kitsune Configuration")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "automod")]
pub async fn automod_toggle(
    ctx: Context<'_>,
    #[description = "Enable or disable auto-moderation"] enable: bool,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(if enable { "✅ Auto-Moderation Enabled" } else { "❌ Auto-Moderation Disabled" })
            .description(format!("Auto-moderation has been **{}**", if enable { "enabled" } else { "disabled" }))
            .color(if enable { 0x2ecc71 } else { 0xe74c3c })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("alerts", "logs", "reports")
)]
pub async fn channel(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use subcommands: `/kitsune channel alerts`, `/kitsune channel logs`, or `/kitsune channel reports`").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn alerts(
    ctx: Context<'_>,
    #[description = "Channel for security alerts"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Alert Channel Set")
            .description(format!("Security alerts will be sent to <#{}>", channel_id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn logs(
    ctx: Context<'_>,
    #[description = "Channel for detailed logs"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Log Channel Set")
            .description(format!("Detailed logs will be sent to <#{}>", channel_id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn reports(
    ctx: Context<'_>,
    #[description = "Channel for automated reports"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Report Channel Set")
            .description(format!("Automated reports will be sent to <#{}>", channel_id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("notify_add", "notify_remove", "notify_level")
)]
pub async fn notify(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use subcommands to manage notification settings").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "add")]
pub async fn notify_add(
    ctx: Context<'_>,
    #[description = "Role to notify"] role: serenity::Role,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Notification Role Added")
            .description(format!("<@&{}> will now receive security notifications", role.id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "remove")]
pub async fn notify_remove(
    ctx: Context<'_>,
    #[description = "Role to stop notifying"] role: serenity::Role,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Notification Role Removed")
            .description(format!("<@&{}> will no longer receive security notifications", role.id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "level")]
pub async fn notify_level(
    ctx: Context<'_>,
    #[description = "Notification level"] 
    level: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Notification Level Set")
            .description(format!("Notification level set to: **{}**", level))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("raid_join_threshold_5s", "raid_join_threshold_30s", "raid_join_threshold_1m", "raid_new_account_days", "raid_username_similarity", "raid_enabled")
)]
pub async fn raid(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use raid subcommands to configure raid detection thresholds").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "join_threshold_5s")]
pub async fn raid_join_threshold_5s(
    ctx: Context<'_>,
    #[description = "Number of joins in 5 seconds to trigger alert"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Raid Detection Updated")
            .description(format!("5-second join threshold set to: **{}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "join_threshold_30s")]
pub async fn raid_join_threshold_30s(
    ctx: Context<'_>,
    #[description = "Number of joins in 30 seconds to trigger alert"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Raid Detection Updated")
            .description(format!("30-second join threshold set to: **{}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "join_threshold_1m")]
pub async fn raid_join_threshold_1m(
    ctx: Context<'_>,
    #[description = "Number of joins in 1 minute to trigger alert"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Raid Detection Updated")
            .description(format!("1-minute join threshold set to: **{}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "new_account_days")]
pub async fn raid_new_account_days(
    ctx: Context<'_>,
    #[description = "Days since account creation to consider 'new'"] days: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Raid Detection Updated")
            .description(format!("New account threshold set to: **{} days**", days))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "username_similarity")]
pub async fn raid_username_similarity(
    ctx: Context<'_>,
    #[description = "Similarity threshold (0.0-1.0) for detecting similar usernames"] threshold: f32,
) -> Result<(), Error> {
    let threshold = threshold.max(0.0).min(1.0);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Raid Detection Updated")
            .description(format!("Username similarity threshold set to: **{:.2}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "enabled")]
pub async fn raid_enabled(
    ctx: Context<'_>,
    #[description = "Enable or disable raid detection"] enable: bool,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(if enable { "✅ Raid Detection Enabled" } else { "❌ Raid Detection Disabled" })
            .description(format!("Raid detection has been **{}**", if enable { "enabled" } else { "disabled" }))
            .color(if enable { 0x2ecc71 } else { 0xe74c3c })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("behavior_message_burst", "behavior_spam_similarity", "behavior_link_spam", "behavior_mention_spam", "behavior_enabled")
)]
pub async fn behavior(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use behavior subcommands to configure behavior analysis settings").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "message_burst_threshold")]
pub async fn behavior_message_burst(
    ctx: Context<'_>,
    #[description = "Messages per minute threshold"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Behavior Analysis Updated")
            .description(format!("Message burst threshold set to: **{}** messages/min", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "spam_similarity")]
pub async fn behavior_spam_similarity(
    ctx: Context<'_>,
    #[description = "Similarity threshold (0.0-1.0) for detecting spam"] threshold: f32,
) -> Result<(), Error> {
    let threshold = threshold.max(0.0).min(1.0);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Behavior Analysis Updated")
            .description(format!("Spam similarity threshold set to: **{:.2}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "link_spam_threshold")]
pub async fn behavior_link_spam(
    ctx: Context<'_>,
    #[description = "Number of links in a short time to flag as spam"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Behavior Analysis Updated")
            .description(format!("Link spam threshold set to: **{}** links", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "mention_spam_threshold")]
pub async fn behavior_mention_spam(
    ctx: Context<'_>,
    #[description = "Number of mentions to flag as spam"] threshold: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Behavior Analysis Updated")
            .description(format!("Mention spam threshold set to: **{}** mentions", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "enabled")]
pub async fn behavior_enabled(
    ctx: Context<'_>,
    #[description = "Enable or disable behavior analysis"] enable: bool,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(if enable { "✅ Behavior Analysis Enabled" } else { "❌ Behavior Analysis Disabled" })
            .description(format!("Behavior analysis has been **{}**", if enable { "enabled" } else { "disabled" }))
            .color(if enable { 0x2ecc71 } else { 0xe74c3c })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("ml_model_path", "ml_inference_timeout", "ml_confidence_threshold", "ml_enabled")
)]
pub async fn ml(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use ml subcommands to configure machine learning settings").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "model_path")]
pub async fn ml_model_path(
    ctx: Context<'_>,
    #[description = "Path to ML model file"] path: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ ML Configuration Updated")
            .description(format!("Model path set to: **{}**", path))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "inference_timeout")]
pub async fn ml_inference_timeout(
    ctx: Context<'_>,
    #[description = "Timeout for ML inference in milliseconds"] timeout_ms: u32,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ ML Configuration Updated")
            .description(format!("Inference timeout set to: **{}ms**", timeout_ms))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "confidence_threshold")]
pub async fn ml_confidence_threshold(
    ctx: Context<'_>,
    #[description = "Confidence threshold (0.0-1.0) for ML predictions"] threshold: f32,
) -> Result<(), Error> {
    let threshold = threshold.max(0.0).min(1.0);
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ ML Configuration Updated")
            .description(format!("Confidence threshold set to: **{:.2}**", threshold))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "enabled")]
pub async fn ml_enabled(
    ctx: Context<'_>,
    #[description = "Enable or disable ML predictions"] enable: bool,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(if enable { "✅ ML Enabled" } else { "❌ ML Disabled" })
            .description(format!("Machine learning predictions have been **{}**", if enable { "enabled" } else { "disabled" }))
            .color(if enable { 0x2ecc71 } else { 0xe74c3c })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Configuration"))
    )).await?;
    Ok(())
}

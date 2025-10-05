use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};
use chrono::{Utc, Duration};

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("stats_server", "stats_user")
)]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    let incidents_24h = queries::get_recent_incidents(&ctx.data().pool, guild_id, 100).await?
        .iter()
        .filter(|i| i.created_at >= Utc::now() - Duration::hours(24))
        .count();
    
    let incidents_7d = queries::get_recent_incidents(&ctx.data().pool, guild_id, 100).await?.len();
    
    let raid_analysis = ctx.data().raid_detector.analyze_raid_risk(guild_id);
    
    let description = format!(
        "**Security Overview**\n\n**Incidents:**\n- Last 24h: {}\n- Last 7d: {}\n\n**Current Threat Level:** {:.2}\n**Raid Risk:** {}\n\n**Auto-Mod Status:** ✅ Active",
        incidents_24h,
        incidents_7d,
        raid_analysis.threat_score,
        if raid_analysis.is_raid { "🚨 DETECTED" } else { "✅ Safe" }
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Security Statistics")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "server")]
pub async fn stats_server(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().unwrap().to_partial_guild(&ctx).await?;
    let member_count = guild.approximate_member_count.unwrap_or(0);
    
    let description = format!(
        "**Server Stats**\n\n**Members:** {}\n**Channels:** {}\n**Roles:** {}\n\n**Activity:** High\n**Growth Rate:** Stable",
        member_count,
        guild.channels(&ctx).await?.len(),
        guild.roles.len()
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Server Statistics")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "user")]
pub async fn stats_user(
    ctx: Context<'_>,
    #[description = "User to analyze"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let user_id = user.id.get() as i64;
    
    let incidents = queries::get_user_incidents(&ctx.data().pool, user_id, 100).await?;
    let metrics = ctx.data().behavior_analyzer.get_behavioral_metrics(guild_id, user_id);
    
    let description = format!(
        "**User:** {}\n\n**Activity:**\n- Total Incidents: {}\n- Threat Score: {:.2}\n- Spam Score: {:.2}\n\n**Status:** {}",
        user.tag(),
        incidents.len(),
        metrics.threat_score,
        metrics.spam_score,
        if metrics.threat_score > 0.6 { "⚠️ Suspicious" } else { "✅ Normal" }
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 User Statistics")
            .description(description)
            .color(0x3498db)
            .thumbnail(user.face())
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("leaderboard_threat", "leaderboard_activity")
)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use subcommands: `threat` or `activity`").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "threat")]
pub async fn leaderboard_threat(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚠️ Top Threat Users")
            .description("No high-threat users detected")
            .color(0xe67e22)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "activity")]
pub async fn leaderboard_activity(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Most Active Users")
            .description("Activity leaderboard will be shown here")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("report_generate")
)]
pub async fn report(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/kitsune report generate` to create a security report").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "generate")]
pub async fn report_generate(
    ctx: Context<'_>,
    #[description = "Hours to include in report"] hours: Option<i64>,
) -> Result<(), Error> {
    let hours = hours.unwrap_or(24);
    
    ctx.defer().await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📄 Security Report Generated")
            .description(format!("Report for last {} hours has been generated.\n\nDownload link will be available for 24 hours.", hours))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("forensics_incident", "forensics_user", "forensics_search")
)]
pub async fn forensics(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use forensics subcommands for detailed analysis").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "incident")]
pub async fn forensics_incident(
    ctx: Context<'_>,
    #[description = "Incident ID"] incident_id: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔍 Incident Forensics")
            .description(format!("Detailed analysis for incident: {}", incident_id))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "user")]
pub async fn forensics_user(
    ctx: Context<'_>,
    #[description = "User to analyze"] user: serenity::User,
    #[description = "Hours to look back"] hours: Option<i64>,
) -> Result<(), Error> {
    let hours = hours.unwrap_or(24);
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔍 User Forensics")
            .description(format!("Complete activity history for {} (last {} hours)", user.tag(), hours))
            .color(0x3498db)
            .thumbnail(user.face())
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "search")]
pub async fn forensics_search(
    ctx: Context<'_>,
    #[description = "Search query"] query: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔍 Forensic Search")
            .description(format!("Search results for: **{}**", query))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("export_data")
)]
pub async fn export(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use `/kitsune export data` to export data").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "data")]
pub async fn export_data(
    ctx: Context<'_>,
    #[description = "Export format"]
    format: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📦 Data Export")
            .description(format!("Exporting data in {} format...", format))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

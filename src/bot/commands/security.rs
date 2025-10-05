use poise::serenity_prelude as serenity;
use crate::database::{queries, models::ThreatLevel};
use crate::bot::{Context, Error};
use chrono::{Utc, Duration};

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    let guild = queries::get_guild(&ctx.data().pool, guild_id).await?;
    let recent_incidents = queries::get_recent_incidents(&ctx.data().pool, guild_id, 10).await?;
    
    let raid_analysis = ctx.data().raid_detector.analyze_raid_risk(guild_id);
    
    let threat_level = ThreatLevel::from_score(raid_analysis.threat_score);
    let lockdown = guild.map(|g| g.lockdown_active).unwrap_or(false);
    
    let description = format!(
        "**Threat Level:** {} ({:.2})\n**Lockdown:** {}\n**Recent Incidents:** {}\n\n**Raid Detection:**\n- 5s: {} joins\n- 30s: {} joins\n- 1m: {} joins\n- 5m: {} joins\n\n**Analysis:**\n{}",
        threat_level.as_str(),
        raid_analysis.threat_score,
        if lockdown { "🔒 Active" } else { "✅ Inactive" },
        recent_incidents.len(),
        raid_analysis.join_rate_5s,
        raid_analysis.join_rate_30s,
        raid_analysis.join_rate_1m,
        raid_analysis.join_rate_5m,
        if raid_analysis.reasons.is_empty() {
            "No threats detected".to_string()
        } else {
            raid_analysis.reasons.join("\n- ")
        }
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 Kitsune Security Status")
            .description(description)
            .color(threat_level.color())
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn scan(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    ctx.defer().await?;
    
    let guild = ctx.guild_id().unwrap().to_partial_guild(&ctx).await?;
    let members = guild.members(&ctx, None, None).await?;
    
    let mut suspicious_count = 0;
    let mut high_threat_users = Vec::new();
    
    for member in members.iter().take(100) {
        let user_id = member.user.id.get() as i64;
        let metrics = ctx.data().behavior_analyzer.get_behavioral_metrics(guild_id, user_id);
        let honeypot_catches = ctx.data().honeypot.get_user_catches(guild_id, user_id);
        
        let threat_score = metrics.threat_score + (honeypot_catches.len() as f32 * 0.2);
        
        if threat_score > 0.6 {
            suspicious_count += 1;
            if threat_score > 0.8 {
                high_threat_users.push((member.user.tag(), threat_score));
            }
        }
    }
    
    high_threat_users.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    high_threat_users.truncate(5);
    
    let threat_list = if high_threat_users.is_empty() {
        "None detected".to_string()
    } else {
        high_threat_users.iter()
            .map(|(tag, score)| format!("• {} ({:.2})", tag, score))
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    let description = format!(
        "**Scanned:** {} members\n**Suspicious:** {}\n**High Threat:** {}\n\n**Top Threats:**\n{}",
        members.len().min(100),
        suspicious_count,
        high_threat_users.len(),
        threat_list
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔍 Full Server Scan Complete")
            .description(description)
            .color(if high_threat_users.is_empty() { 0x2ecc71 } else { 0xe67e22 })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn check(
    ctx: Context<'_>,
    #[description = "User to check"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let user_id = user.id.get() as i64;
    
    let db_user = queries::get_user(&ctx.data().pool, user_id).await?;
    let incidents = queries::get_user_incidents(&ctx.data().pool, user_id, 5).await?;
    
    let behavioral_metrics = ctx.data().behavior_analyzer.get_behavioral_metrics(guild_id, user_id);
    let honeypot_catches = ctx.data().honeypot.get_user_catches(guild_id, user_id);
    
    let threat_level = ThreatLevel::from_score(behavioral_metrics.threat_score);
    
    let description = format!(
        "**User:** {}\n**ID:** {}\n**Global Reputation:** {}\n**Total Incidents:** {}\n\n**Threat Level:** {} ({:.2})\n\n**Behavioral Metrics:**\n- Spam Score: {:.2}\n- Link Density: {:.2}\n- Mention Ratio: {:.2}\n- Caps Ratio: {:.2}\n\n**Honeypot Catches:** {}\n**Recent Incidents:** {}",
        user.tag(),
        user_id,
        db_user.as_ref().map(|u| u.global_reputation).unwrap_or(0),
        db_user.as_ref().map(|u| u.total_incidents).unwrap_or(0),
        threat_level.as_str(),
        behavioral_metrics.threat_score,
        behavioral_metrics.spam_score,
        behavioral_metrics.link_density,
        behavioral_metrics.mention_ratio,
        behavioral_metrics.caps_ratio,
        honeypot_catches.len(),
        incidents.len()
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("🔍 User Analysis: {}", user.tag()))
            .description(description)
            .color(threat_level.color())
            .thumbnail(user.face())
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn analyze(
    ctx: Context<'_>,
    #[description = "Time range in hours (default 24)"] hours: Option<i64>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let hours = hours.unwrap_or(24).max(1).min(168);
    
    let incidents = queries::get_recent_incidents(&ctx.data().pool, guild_id, 50).await?;
    
    let cutoff = Utc::now() - Duration::hours(hours);
    let recent_incidents: Vec<_> = incidents.iter()
        .filter(|i| i.created_at >= cutoff)
        .collect();
    
    let high_severity = recent_incidents.iter()
        .filter(|i| i.severity == "high" || i.severity == "critical")
        .count();
    
    let description = format!(
        "**Time Range:** Last {} hours\n**Total Incidents:** {}\n**High/Critical:** {}\n\n**Analysis:**\n{}",
        hours,
        recent_incidents.len(),
        high_severity,
        if recent_incidents.is_empty() {
            "No suspicious activity detected in this time range."
        } else {
            "Review incidents with `/kitsune report` for details."
        }
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📊 Activity Analysis")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation")]
pub async fn reputation_cmd(
    ctx: Context<'_>,
    #[description = "User to check reputation"] user: serenity::User,
) -> Result<(), Error> {
    let user_id = user.id.get() as i64;
    
    let db_user = queries::get_user(&ctx.data().pool, user_id).await?;
    let incidents = queries::get_user_incidents(&ctx.data().pool, user_id, 10).await?;
    
    let reputation = db_user.as_ref().map(|u| u.global_reputation).unwrap_or(0);
    let total_incidents = db_user.as_ref().map(|u| u.total_incidents).unwrap_or(0);
    
    let rep_level = if reputation >= 50 {
        "✅ Trusted"
    } else if reputation >= 0 {
        "⚪ Neutral"
    } else if reputation >= -50 {
        "⚠️ Suspicious"
    } else {
        "🚫 Malicious"
    };
    
    let description = format!(
        "**User:** {}\n**ID:** {}\n\n**Global Reputation:** {} ({})\n**Total Incidents:** {}\n**Recent Incidents:** {}",
        user.tag(),
        user_id,
        reputation,
        rep_level,
        total_incidents,
        incidents.len()
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌐 Global Reputation")
            .description(description)
            .color(if reputation >= 0 { 0x2ecc71 } else { 0xe74c3c })
            .thumbnail(user.face())
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

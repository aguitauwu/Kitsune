use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};
use serde_json::json;

#[poise::command( slash_command, guild_only = true, required_permissions = "BAN_MEMBERS")]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason for ban"] reason: Option<String>,
    #[description = "Days of messages to delete (0-7)"] delete_days: Option<u8>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;
    let user_id = user.id;
    let moderator_id = ctx.author().id.get() as i64;
    let delete_days = delete_days.unwrap_or(1).min(7);
    let reason_str = reason.clone().unwrap_or_else(|| "No reason provided".to_string());
    
    let member = guild_id.member(&ctx, user_id).await?;
    member.ban_with_reason(&ctx, delete_days, &reason_str).await?;
    
    queries::create_incident(
        &ctx.data().pool,
        guild_id.get() as i64,
        user_id.get() as i64,
        "ban",
        "critical",
        1.0,
        json!({"reason": reason_str, "moderator": moderator_id}),
        Some("ban")
    ).await?;
    
    queries::update_user_reputation(&ctx.data().pool, user_id.get() as i64, -20).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔨 User Banned")
            .description(format!("**User:** {}\n**Reason:** {}\n**Messages Deleted:** {} days", user.tag(), reason_str, delete_days))
            .color(0xe74c3c)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "KICK_MEMBERS")]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
    #[description = "Reason for kick"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;
    let user_id = user.id;
    let moderator_id = ctx.author().id.get() as i64;
    let reason_str = reason.clone().unwrap_or_else(|| "No reason provided".to_string());
    
    let member = guild_id.member(&ctx, user_id).await?;
    member.kick_with_reason(&ctx, &reason_str).await?;
    
    queries::create_incident(
        &ctx.data().pool,
        guild_id.get() as i64,
        user_id.get() as i64,
        "kick",
        "high",
        0.8,
        json!({"reason": reason_str, "moderator": moderator_id}),
        Some("kick")
    ).await?;
    
    queries::update_user_reputation(&ctx.data().pool, user_id.get() as i64, -10).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("👢 User Kicked")
            .description(format!("**User:** {}\n**Reason:** {}", user.tag(), reason_str))
            .color(0xe67e22)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "MODERATE_MEMBERS")]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "User to timeout"] user: serenity::User,
    #[description = "Duration in minutes"] duration: u64,
    #[description = "Reason for timeout"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;
    let user_id = user.id;
    let reason_str = reason.clone().unwrap_or_else(|| "No reason provided".to_string());
    
    let mut member = guild_id.member(&ctx, user_id).await?;
    let timeout_until = serenity::Timestamp::from_unix_timestamp(
        chrono::Utc::now().timestamp() + (duration as i64 * 60)
    )?;
    
    member.disable_communication_until_datetime(&ctx, timeout_until).await?;
    
    queries::create_incident(
        &ctx.data().pool,
        guild_id.get() as i64,
        user_id.get() as i64,
        "timeout",
        "medium",
        0.6,
        json!({"reason": reason_str, "duration_minutes": duration}),
        Some("timeout")
    ).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⏱️ User Timed Out")
            .description(format!("**User:** {}\n**Duration:** {} minutes\n**Reason:** {}", user.tag(), duration, reason_str))
            .color(0xf39c12)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "MODERATE_MEMBERS")]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "User to warn"] user: serenity::User,
    #[description = "Reason for warning"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;
    let user_id = user.id;
    let moderator_id = ctx.author().id.get() as i64;
    
    queries::create_incident(
        &ctx.data().pool,
        guild_id.get() as i64,
        user_id.get() as i64,
        "warning",
        "low",
        0.3,
        json!({"reason": reason, "moderator": moderator_id}),
        Some("warning")
    ).await?;
    
    let user_incidents = queries::get_user_incidents(&ctx.data().pool, user_id.get() as i64, 10).await?;
    let warnings = user_incidents.iter().filter(|i| i.incident_type == "warning").count();
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⚠️ User Warned")
            .description(format!("**User:** {}\n**Reason:** {}\n**Total Warnings:** {}", user.tag(), reason, warnings))
            .color(0xf39c12)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "BAN_MEMBERS")]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "User ID to unban"] user_id: String,
    #[description = "Reason for unban"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;
    let user_id_u64: u64 = user_id.parse().map_err(|_| "Invalid user ID")?;
    let user_id_obj = serenity::UserId::new(user_id_u64);
    let reason_str = reason.unwrap_or_else(|| "No reason provided".to_string());
    
    guild_id.unban(&ctx, user_id_obj).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ User Unbanned")
            .description(format!("**User ID:** {}\n**Reason:** {}", user_id, reason_str))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn pardon(
    ctx: Context<'_>,
    #[description = "User to pardon"] user: serenity::User,
) -> Result<(), Error> {
    let user_id = user.id.get() as i64;
    
    queries::update_user_reputation(&ctx.data().pool, user_id, 20).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🕊️ User Pardoned")
            .description(format!("**User:** {}\nWarnings cleared and reputation improved.", user.tag()))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

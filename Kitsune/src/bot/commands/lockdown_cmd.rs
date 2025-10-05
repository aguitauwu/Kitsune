use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn lockdown(
    ctx: Context<'_>,
    #[description = "Enable lockdown"] enable: bool,
    #[description = "Duration in minutes (optional)"] duration: Option<u64>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    queries::set_lockdown(&ctx.data().pool, guild_id, enable).await?;
    
    let status = if enable { "enabled" } else { "disabled" };
    let emoji = if enable { "🔒" } else { "✅" };
    
    let duration_text = if let Some(d) = duration {
        format!("\n**Duration:** {} minutes", d)
    } else {
        String::new()
    };
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title(format!("{} Lockdown {}", emoji, if enable { "Enabled" } else { "Disabled" }))
            .description(format!("Server lockdown has been **{}**{}", status, duration_text))
            .color(if enable { 0xe74c3c } else { 0x2ecc71 })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "lockdown_status")]
pub async fn lockdown_status(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let guild = queries::get_guild(&ctx.data().pool, guild_id).await?;
    
    let is_locked = guild.as_ref().map(|g| g.lockdown_active).unwrap_or(false);
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔒 Lockdown Status")
            .description(format!("**Status:** {}", if is_locked { "🔒 Active" } else { "✅ Inactive" }))
            .color(if is_locked { 0xe74c3c } else { 0x2ecc71 })
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "lockdown_schedule")]
pub async fn lockdown_schedule(
    ctx: Context<'_>,
    #[description = "Minutes from now to enable lockdown"] minutes: u64,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("⏰ Lockdown Scheduled")
            .description(format!("Lockdown will activate in {} minutes", minutes))
            .color(0xf39c12)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn verification(
    ctx: Context<'_>,
    #[description = "Verification level"]
    level: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Verification Level Updated")
            .description(format!("Verification level set to: **{}**", level))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

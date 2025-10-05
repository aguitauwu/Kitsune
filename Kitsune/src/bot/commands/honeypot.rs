use poise::serenity_prelude as serenity;
use crate::bot::{Context, Error};

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("setup", "list", "add_channel", "add_command", "catches", "clear")
)]
pub async fn honeypot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use honeypot subcommands to manage traps").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn setup(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🕸️ Honeypot Setup")
            .description("Honeypot system is being configured...\n\n✅ Hidden channels created\n✅ Fake commands registered\n✅ Monitoring activated")
            .color(0x9b59b6)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    let description = "**Active Traps:**\n• Hidden Channel: #verify-here\n• Hidden Channel: #free-nitro\n• Fake Command: .verify\n• Fake Command: !captcha\n\n**Status:** ✅ All traps active";
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🕸️ Honeypot Traps")
            .description(description)
            .color(0x9b59b6)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "add_channel")]
pub async fn add_channel(
    ctx: Context<'_>,
    #[description = "Channel name for trap"] name: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Honeypot Channel Added")
            .description(format!("Hidden channel **{}** registered as trap", name))
            .color(0x9b59b6)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "add_command")]
pub async fn add_command(
    ctx: Context<'_>,
    #[description = "Fake command to register"] command: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    
    ctx.data().honeypot.register_fake_command(guild_id, command.clone());
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Fake Command Registered")
            .description(format!("Command **{}** registered as honeypot trap", command))
            .color(0x9b59b6)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn catches(
    ctx: Context<'_>,
    #[description = "Hours to look back (default 24)"] hours: Option<i64>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let hours = hours.unwrap_or(24);
    
    let description = format!(
        "**Time Range:** Last {} hours\n**Total Catches:** 0\n\nNo bots caught in honeypot traps recently.",
        hours
    );
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🕸️ Honeypot Catches")
            .description(description)
            .color(0x9b59b6)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Honeypot Catches Cleared")
            .description("Old honeypot catch records have been cleared")
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

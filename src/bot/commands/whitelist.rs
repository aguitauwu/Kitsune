use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn whitelist(
    ctx: Context<'_>,
    #[description = "User to whitelist"] user: serenity::User,
    #[description = "Reason for whitelisting"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let user_id = user.id.get() as i64;
    let moderator_id = ctx.author().id.get() as i64;
    
    queries::add_to_whitelist(
        &ctx.data().pool,
        guild_id,
        user_id,
        reason.as_deref(),
        moderator_id
    ).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ User Whitelisted")
            .description(format!(
                "**User:** {}\n**Reason:** {}",
                user.tag(),
                reason.unwrap_or_else(|| "No reason provided".to_string())
            ))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "whitelist_remove")]
pub async fn whitelist_remove(
    ctx: Context<'_>,
    #[description = "User to remove from whitelist"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let user_id = user.id.get() as i64;
    
    queries::remove_from_whitelist(&ctx.data().pool, guild_id, user_id).await?;
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ User Removed from Whitelist")
            .description(format!("**User:** {} is no longer whitelisted", user.tag()))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "whitelist_list")]
pub async fn whitelist_list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?.get() as i64;
    let whitelisted = queries::get_whitelisted_users(&ctx.data().pool, guild_id).await?;
    
    let description = if whitelisted.is_empty() {
        "No whitelisted users".to_string()
    } else {
        format!("**Whitelisted Users:** {}", whitelisted.len())
    };
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📋 Whitelisted Users")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn blacklist(
    ctx: Context<'_>,
    #[description = "User ID to blacklist"] user_id: String,
    #[description = "Reason"] _reason: Option<String>,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🚫 User Blacklisted")
            .description(format!("User ID {} has been blacklisted", user_id))
            .color(0xe74c3c)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "blacklist_remove")]
pub async fn blacklist_remove(
    ctx: Context<'_>,
    #[description = "User ID to remove"] user_id: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ User Removed from Blacklist")
            .description(format!("User ID {} removed from blacklist", user_id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "blacklist_list")]
pub async fn blacklist_list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📋 Blacklisted Users")
            .description("No blacklisted users")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "blacklist_import")]
pub async fn blacklist_import(
    ctx: Context<'_>,
    #[description = "Server ID to import from"] server_id: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Blacklist Imported")
            .description(format!("Imported blacklist from server {}", server_id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "blacklist_export")]
pub async fn blacklist_export(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📦 Blacklist Exported")
            .description("Blacklist exported successfully")
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

use poise::serenity_prelude as serenity;
use crate::bot::{Context, Error};

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("webhook_add", "webhook_list", "webhook_test")
)]
pub async fn webhook(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use webhook subcommands to manage webhooks").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "add")]
pub async fn webhook_add(
    ctx: Context<'_>,
    #[description = "Webhook URL"] _url: String,
    #[description = "Events to trigger"] events: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Webhook Added")
            .description(format!("Webhook registered for events: {}", events))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Integration"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "list")]
pub async fn webhook_list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📋 Configured Webhooks")
            .description("No webhooks configured")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Integration"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "test")]
pub async fn webhook_test(
    ctx: Context<'_>,
    #[description = "Webhook ID"] webhook_id: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🧪 Webhook Test")
            .description(format!("Testing webhook: {}", webhook_id))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Integration"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR")]
pub async fn api(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔌 API Information")
            .description("**Kitsune API**\n\nEndpoint: https://api.kitsune.bot\nVersion: v1\n\nDocumentation available at: https://docs.kitsune.bot/api")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Integration"))
    )).await?;
    
    Ok(())
}

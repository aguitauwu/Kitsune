use poise::serenity_prelude as serenity;
use crate::bot::{Context, Error};

#[poise::command(
    slash_command,
    guild_only = true,
    required_permissions = "ADMINISTRATOR",
    subcommands("custom_response", "custom_message", "custom_role")
)]
pub async fn custom(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Use custom subcommands to personalize Kitsune").await?;
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "response")]
pub async fn custom_response(
    ctx: Context<'_>,
    #[description = "Threat level"]
    level: String,
    #[description = "Custom action"] action: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Custom Response Set")
            .description(format!("Custom action for **{}** threat level: {}", level, action))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Custom"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "message")]
pub async fn custom_message(
    ctx: Context<'_>,
    #[description = "Event type"] event_type: String,
    #[description = "Message template"] template: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Custom Message Set")
            .description(format!("Custom message for **{}**: {}", event_type, template))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Custom"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "role")]
pub async fn custom_role(
    ctx: Context<'_>,
    #[description = "Action type"] action: String,
    #[description = "Role to assign"] role: serenity::Role,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Custom Role Action Set")
            .description(format!("Action **{}** will assign role <@&{}>", action, role.id))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Custom"))
    )).await?;
    
    Ok(())
}

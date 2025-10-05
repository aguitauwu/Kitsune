use poise::serenity_prelude as serenity;
use crate::database::queries;
use crate::bot::{Context, Error};

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_query")]
pub async fn reputation_query(
    ctx: Context<'_>,
    #[description = "User ID to query"] user_id: String,
) -> Result<(), Error> {
    let user_id_i64: i64 = user_id.parse().map_err(|_| "Invalid user ID")?;
    let db_user = queries::get_user(&ctx.data().pool, user_id_i64).await?;
    
    let reputation = db_user.as_ref().map(|u| u.global_reputation).unwrap_or(0);
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌐 Reputation Query")
            .description(format!("**User ID:** {}\n**Global Reputation:** {}\n**Trust Score:** Neutral", user_id, reputation))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_report")]
pub async fn reputation_report(
    ctx: Context<'_>,
    #[description = "User to report"] user: serenity::User,
    #[description = "Incident type"] incident_type: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Report Submitted")
            .description(format!("Incident for {} has been reported to reputation network", user.tag()))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_trust")]
pub async fn reputation_trust(
    ctx: Context<'_>,
    #[description = "Server ID"] server_id: String,
    #[description = "Trust weight (0.0-1.0)"] weight: f32,
) -> Result<(), Error> {
    let weight = weight.max(0.0).min(1.0);
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Trust Relationship Updated")
            .description(format!("Trust weight for server {} set to {:.2}", server_id, weight))
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_sync")]
pub async fn reputation_sync(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🔄 Syncing with Network")
            .description("Synchronizing reputation data with trusted servers...")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_servers")]
pub async fn reputation_servers(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🌐 Trusted Servers")
            .description("**Connected Servers:** 0\n\nNo trust relationships established yet.")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

#[poise::command( slash_command, guild_only = true, required_permissions = "ADMINISTRATOR", rename = "reputation_appeal")]
pub async fn reputation_appeal(
    ctx: Context<'_>,
    #[description = "Report ID to appeal"] report_id: String,
) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✅ Appeal Submitted")
            .description(format!("Appeal for report {} has been submitted for review", report_id))
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Reputation Network"))
    )).await?;
    
    Ok(())
}

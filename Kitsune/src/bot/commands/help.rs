use poise::serenity_prelude as serenity;
use crate::bot::{Context, Error};

#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to get help for"] command: Option<String>,
) -> Result<(), Error> {
    let description = if let Some(cmd) = command {
        format!("Help for command: **{}**\n\nUse `/kitsune {}` to execute this command.", cmd, cmd)
    } else {
        "**🦊 Kitsune Guardian Fox - Command Categories**\n\n\
        **🛡️ Security:** `status`, `scan`, `check`, `analyze`, `reputation`\n\
        **🚨 Moderation:** `ban`, `kick`, `timeout`, `warn`, `unban`, `pardon`\n\
        **⚙️ Configuration:** `config`, `channel`, `notify`\n\
        **🕸️ Honeypot:** `honeypot setup/list/catches`\n\
        **📊 Stats:** `stats`, `leaderboard`, `report`, `forensics`\n\
        **🌐 Reputation:** `reputation query/report/trust/sync`\n\
        **🔒 Lockdown:** `lockdown`, `lockdown_status`, `verification`\n\
        **👥 Lists:** `whitelist`, `blacklist` (add/remove/list)\n\
        **🧪 Testing:** `test`, `debug`, `health`\n\
        **📚 Info:** `help`, `about`, `docs`, `invite`\n\
        **📈 Analytics:** `analytics`, `predict`, `compare`\n\
        **💾 Backup:** `backup` (create/restore/list)\n\
        **🎨 Custom:** `custom`, `webhook`, `api`\n\n\
        Use `/help <command>` for detailed help on a specific command.".to_string()
    };
    
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📚 Kitsune Help")
            .description(description)
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox v1.0"))
    )).await?;
    
    Ok(())
}

#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("🦊 About Kitsune")
            .description("**Kitsune Guardian Fox**\nAdvanced Discord Security Bot\n\n**Version:** 1.0.0\n**Language:** Rust 🦀\n**Features:**\n✅ Real-time Raid Detection\n✅ Behavioral Analysis\n✅ Honeypot Traps\n✅ Auto-Moderation\n✅ Forensic Logging\n✅ Reputation Network\n\n**Created by:** Kitsune Team\n**License:** MIT")
            .color(0xe67e22)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(slash_command)]
pub async fn docs(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("📖 Documentation")
            .description("**Kitsune Documentation**\n\nFull documentation is available at:\nhttps://docs.kitsune.bot\n\nQuick start guide, API reference, and tutorials included.")
            .color(0x3498db)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

#[poise::command(slash_command)]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(poise::CreateReply::default().embed(
        serenity::CreateEmbed::new()
            .title("✉️ Invite Kitsune")
            .description("Add Kitsune to your server:\n\n[Click here to invite](https://discord.com/api/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands)\n\n**Required Permissions:**\n• Manage Server\n• Ban Members\n• Kick Members\n• Manage Roles\n• Manage Channels")
            .color(0x2ecc71)
            .footer(serenity::CreateEmbedFooter::new("Kitsune Guardian Fox"))
    )).await?;
    
    Ok(())
}

use poise::serenity_prelude as serenity;
use chrono::Utc;
use serde_json::json;

use crate::database::{queries, models::ThreatLevel};
use crate::security::{JoinEvent, threat_calculator::ThreatCalculator, auto_mod::ModAction};

use super::Data;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, super::Error>,
    data: &Data,
) -> Result<(), super::Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            handle_member_join(ctx, new_member, data).await?;
        }
        serenity::FullEvent::Message { new_message } => {
            handle_message(ctx, new_message, data).await?;
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            handle_guild_create(guild, data).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_guild_create(guild: &serenity::Guild, data: &Data) -> Result<(), super::Error> {
    let guild_id = guild.id.get() as i64;
    let owner_id = guild.owner_id.get() as i64;
    
    queries::upsert_guild(&data.pool, guild_id, &guild.name, owner_id).await?;
    
    tracing::info!("Registered guild: {} ({})", guild.name, guild_id);
    Ok(())
}

async fn handle_member_join(
    ctx: &serenity::Context,
    member: &serenity::Member,
    data: &Data,
) -> Result<(), super::Error> {
    let guild_id = member.guild_id.get() as i64;
    let user_id = member.user.id.get() as i64;
    
    queries::upsert_user(
        &data.pool,
        user_id,
        &member.user.name,
        member.user.discriminator.as_ref().map(|d| d.to_string()).as_deref()
    ).await?;
    
    queries::get_or_create_behavior_profile(&data.pool, guild_id, user_id).await?;
    
    let is_whitelisted = queries::is_whitelisted(&data.pool, guild_id, user_id).await?;
    if is_whitelisted {
        return Ok(());
    }
    
    let join_event = JoinEvent {
        user_id,
        username: member.user.name.clone(),
        discriminator: member.user.discriminator.as_ref().map(|d| d.to_string()),
        account_created: member.user.id.created_at().to_utc(),
        join_time: Utc::now(),
        avatar_hash: member.user.avatar.as_ref().map(|a| a.to_string()),
    };
    
    data.raid_detector.record_join(guild_id, join_event.clone());
    
    let raid_analysis = data.raid_detector.analyze_raid_risk(guild_id);
    
    queries::log_forensic_event(
        &data.pool,
        guild_id,
        Some(user_id),
        "member_join",
        None,
        json!({
            "username": join_event.username,
            "account_age_days": (Utc::now() - join_event.account_created).num_days(),
        }),
        raid_analysis.threat_score,
        vec!["join".to_string()]
    ).await?;
    
    if raid_analysis.is_raid {
        let threat_level = ThreatLevel::from_score(raid_analysis.threat_score);
        
        let action = data.auto_mod.determine_action(raid_analysis.threat_score, threat_level);
        let action_name = action.as_ref().map(|a| match a {
            ModAction::Monitor => "monitor",
            ModAction::Timeout { .. } => "timeout",
            ModAction::Kick { .. } => "kick",
            ModAction::Ban { .. } => "ban",
            ModAction::Lockdown => "lockdown",
        });
        
        queries::create_incident(
            &data.pool,
            guild_id,
            user_id,
            "raid_detection",
            threat_level.as_str(),
            raid_analysis.threat_score,
            json!({
                "join_rate_5s": raid_analysis.join_rate_5s,
                "join_rate_1m": raid_analysis.join_rate_1m,
                "new_account_ratio": raid_analysis.new_account_ratio,
                "username_similarity": raid_analysis.username_similarity,
                "avatar_duplication": raid_analysis.avatar_duplication,
                "reasons": raid_analysis.reasons,
            }),
            action_name
        ).await?;
        
        if let Some(action) = action {
            execute_mod_action(ctx, guild_id, user_id, action, data).await?;
        }
    }
    
    Ok(())
}

async fn handle_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &Data,
) -> Result<(), super::Error> {
    if message.author.bot {
        return Ok(());
    }
    
    let guild_id = match message.guild_id {
        Some(gid) => gid.get() as i64,
        None => return Ok(()),
    };
    
    let user_id = message.author.id.get() as i64;
    let channel_id = message.channel_id.get() as i64;
    
    let is_whitelisted = queries::is_whitelisted(&data.pool, guild_id, user_id).await?;
    if is_whitelisted {
        return Ok(());
    }
    
    if data.honeypot.check_hidden_channel(guild_id, channel_id, user_id) {
        queries::record_honeypot_catch(
            &data.pool,
            guild_id,
            user_id,
            "hidden_channel",
            &format!("channel_{}", channel_id),
            json!({"channel_id": channel_id})
        ).await?;
    }
    
    if data.honeypot.check_fake_command(guild_id, &message.content, user_id) {
        queries::record_honeypot_catch(
            &data.pool,
            guild_id,
            user_id,
            "fake_command",
            &message.content,
            json!({"content": message.content})
        ).await?;
    }
    
    let message_analysis = data.behavior_analyzer.analyze_message(
        guild_id,
        user_id,
        &message.content,
        channel_id
    );
    
    let behavioral_metrics = data.behavior_analyzer.get_behavioral_metrics(guild_id, user_id);
    
    queries::update_behavior_profile(
        &data.pool,
        guild_id,
        user_id,
        json!({
            "spam_score": behavioral_metrics.spam_score,
            "link_density": behavioral_metrics.link_density,
            "mention_ratio": behavioral_metrics.mention_ratio,
            "caps_ratio": behavioral_metrics.caps_ratio,
            "emoji_density": behavioral_metrics.emoji_density,
            "threat_score": behavioral_metrics.threat_score
        })
    ).await?;
    
    let honeypot_multiplier = data.honeypot.get_threat_multiplier(guild_id, user_id);
    let account_age = Utc::now() - message.author.id.created_at().to_utc();
    let is_new_account = account_age.num_days() < data.config.security.new_account_days as i64;
    
    let raid_analysis = data.raid_detector.analyze_raid_risk(guild_id);
    
    let combined_threat = ThreatCalculator::calculate_combined_threat(
        &raid_analysis,
        &behavioral_metrics,
        honeypot_multiplier,
        is_new_account
    );
    
    if combined_threat > data.config.auto_mod.low_threat_threshold {
        queries::log_forensic_event(
            &data.pool,
            guild_id,
            Some(user_id),
            "message",
            Some(&message.content),
            json!({
                "channel_id": channel_id,
                "threat_score": combined_threat,
                "spam_score": behavioral_metrics.spam_score,
                "link_density": behavioral_metrics.link_density,
                "burst_detected": behavioral_metrics.burst_detected,
                "has_links": message_analysis.has_links,
                "mention_count": message_analysis.mention_count,
            }),
            combined_threat,
            vec!["message".to_string(), "threat".to_string()]
        ).await?;
        
        let threat_level = ThreatLevel::from_score(combined_threat);
        
        if combined_threat >= data.config.auto_mod.medium_threat_threshold {
            let honeypot_catches = data.honeypot.get_user_catches(guild_id, user_id);
            let trap_details: Vec<_> = honeypot_catches.iter()
                .map(|c| json!({
                    "trap_type": c.trap_type,
                    "trap_name": c.trap_name,
                    "severity": c.severity,
                }))
                .collect();
            
            let action = data.auto_mod.determine_action(combined_threat, threat_level);
            let action_name = action.as_ref().map(|a| match a {
                ModAction::Monitor => "monitor",
                ModAction::Timeout { .. } => "timeout",
                ModAction::Kick { .. } => "kick",
                ModAction::Ban { .. } => "ban",
                ModAction::Lockdown => "lockdown",
            });
            
            queries::create_incident(
                &data.pool,
                guild_id,
                user_id,
                "behavioral_threat",
                threat_level.as_str(),
                combined_threat,
                json!({
                    "spam_score": behavioral_metrics.spam_score,
                    "burst_detected": behavioral_metrics.burst_detected,
                    "honeypot_multiplier": honeypot_multiplier,
                    "honeypot_traps": trap_details,
                }),
                action_name
            ).await?;
            
            if let Some(action) = action {
                execute_mod_action(ctx, guild_id, user_id, action, data).await?;
            }
        }
    }
    
    Ok(())
}

async fn execute_mod_action(
    ctx: &serenity::Context,
    guild_id: i64,
    user_id: i64,
    action: ModAction,
    data: &Data,
) -> Result<(), super::Error> {
    let guild_id_u64 = serenity::GuildId::new(guild_id as u64);
    let user_id_u64 = serenity::UserId::new(user_id as u64);
    
    match action {
        ModAction::Monitor => {
            tracing::info!("Monitoring user {} in guild {}", user_id, guild_id);
        }
        ModAction::Timeout { duration_minutes } => {
            tracing::info!("Timing out user {} for {} minutes", user_id, duration_minutes);
            
            if let Ok(mut member) = guild_id_u64.member(ctx, user_id_u64).await {
                let until = serenity::Timestamp::from_unix_timestamp(
                    serenity::Timestamp::now().unix_timestamp() + (duration_minutes as i64 * 60)
                ).ok();
                
                if let Some(timestamp) = until {
                    let _ = member.disable_communication_until_datetime(ctx, timestamp).await;
                }
            }
        }
        ModAction::Kick { reason } => {
            tracing::info!("Kicking user {} from guild {}: {}", user_id, guild_id, reason);
            
            let _ = guild_id_u64.kick_with_reason(ctx, user_id_u64, &reason).await;
        }
        ModAction::Ban { reason, delete_days } => {
            tracing::info!("Banning user {} from guild {}: {}", user_id, guild_id, reason);
            
            let _ = guild_id_u64.ban_with_reason(ctx, user_id_u64, delete_days, &reason).await;
            
            let recent_bans = queries::count_recent_bans(&data.pool, guild_id, 60).await.unwrap_or(0);
            let raid_analysis = data.raid_detector.analyze_raid_risk(guild_id);
            
            if data.auto_mod.should_lockdown(raid_analysis.threat_score, recent_bans) {
                tracing::warn!("Auto-lockdown triggered for guild {} - {} recent bans, threat score: {}", 
                    guild_id, recent_bans, raid_analysis.threat_score);
                queries::set_lockdown(&data.pool, guild_id, true).await?;
            }
        }
        ModAction::Lockdown => {
            tracing::warn!("Lockdown triggered for guild {}", guild_id);
            queries::set_lockdown(&data.pool, guild_id, true).await?;
        }
    }
    
    Ok(())
}

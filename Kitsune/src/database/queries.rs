use anyhow::Result;
use sqlx::PgPool;

use super::models::*;

pub async fn upsert_guild(pool: &PgPool, guild_id: i64, name: &str, owner_id: i64) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO guilds (guild_id, name, owner_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (guild_id) 
        DO UPDATE SET name = $2, owner_id = $3, updated_at = NOW()
        "#,
        guild_id,
        name,
        owner_id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_guild(pool: &PgPool, guild_id: i64) -> Result<Option<Guild>> {
    let guild = sqlx::query_as!(
        Guild,
        r#"SELECT * FROM guilds WHERE guild_id = $1"#,
        guild_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(guild)
}

pub async fn upsert_user(pool: &PgPool, user_id: i64, username: &str, discriminator: Option<&str>) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, discriminator, last_seen)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (user_id)
        DO UPDATE SET username = $2, discriminator = $3, last_seen = NOW()
        "#,
        user_id,
        username,
        discriminator
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_user(pool: &PgPool, user_id: i64) -> Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT * FROM users WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}

pub async fn get_or_create_behavior_profile(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64
) -> Result<BehaviorProfile> {
    let profile = sqlx::query_as!(
        BehaviorProfile,
        r#"
        INSERT INTO behavior_profiles (guild_id, user_id, features)
        VALUES ($1, $2, '{}'::jsonb)
        ON CONFLICT (guild_id, user_id)
        DO UPDATE SET updated_at = NOW()
        RETURNING *
        "#,
        guild_id,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(profile)
}

pub async fn update_behavior_profile(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    features: serde_json::Value
) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE behavior_profiles
        SET features = $3, updated_at = NOW()
        WHERE guild_id = $1 AND user_id = $2
        "#,
        guild_id,
        user_id,
        features
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn create_incident(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    incident_type: &str,
    severity: &str,
    threat_score: f32,
    evidence: serde_json::Value,
    action_taken: Option<&str>
) -> Result<Incident> {
    let incident = sqlx::query_as!(
        Incident,
        r#"
        INSERT INTO incidents (guild_id, user_id, incident_type, severity, threat_score, evidence, action_taken)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
        guild_id,
        user_id,
        incident_type,
        severity,
        threat_score,
        evidence,
        action_taken
    )
    .fetch_one(pool)
    .await?;
    
    Ok(incident)
}

pub async fn log_forensic_event(
    pool: &PgPool,
    guild_id: i64,
    user_id: Option<i64>,
    event_type: &str,
    content: Option<&str>,
    metadata: serde_json::Value,
    threat_score: f32,
    tags: Vec<String>
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO forensic_events (guild_id, user_id, event_type, content, metadata, threat_score, tags)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        guild_id,
        user_id,
        event_type,
        content,
        metadata,
        threat_score,
        &tags
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn record_honeypot_catch(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    trap_type: &str,
    trap_name: &str,
    metadata: serde_json::Value
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO honeypot_catches (guild_id, user_id, trap_type, trap_name, metadata)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        guild_id,
        user_id,
        trap_type,
        trap_name,
        metadata
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn is_whitelisted(pool: &PgPool, guild_id: i64, user_id: i64) -> Result<bool> {
    let result = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM whitelisted_users 
            WHERE guild_id = $1 AND user_id = $2
        ) as "exists!"
        "#,
        guild_id,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(result.exists)
}

pub async fn add_to_whitelist(
    pool: &PgPool,
    guild_id: i64,
    user_id: i64,
    reason: Option<&str>,
    added_by: i64
) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO whitelisted_users (guild_id, user_id, reason, added_by)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (guild_id, user_id) DO NOTHING
        "#,
        guild_id,
        user_id,
        reason,
        added_by
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_recent_incidents(pool: &PgPool, guild_id: i64, limit: i64) -> Result<Vec<Incident>> {
    let incidents = sqlx::query_as!(
        Incident,
        r#"
        SELECT * FROM incidents
        WHERE guild_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        guild_id,
        limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(incidents)
}

pub async fn get_user_incidents(pool: &PgPool, user_id: i64, limit: i64) -> Result<Vec<Incident>> {
    let incidents = sqlx::query_as!(
        Incident,
        r#"
        SELECT * FROM incidents
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        user_id,
        limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(incidents)
}

pub async fn set_lockdown(pool: &PgPool, guild_id: i64, active: bool) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE guilds
        SET lockdown_active = $2, updated_at = NOW()
        WHERE guild_id = $1
        "#,
        guild_id,
        active
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn count_recent_bans(pool: &PgPool, guild_id: i64, minutes: i32) -> Result<u32> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM incidents
        WHERE guild_id = $1
        AND action_taken = 'ban'
        AND created_at >= NOW() - ($2 || ' minutes')::INTERVAL
        "#,
        guild_id,
        minutes.to_string()
    )
    .fetch_one(pool)
    .await?;
    
    Ok(result.count.unwrap_or(0) as u32)
}

#[allow(dead_code)]
pub async fn update_incident_action(pool: &PgPool, guild_id: i64, user_id: i64, action: &str) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE incidents
        SET action_taken = $3
        WHERE id = (
            SELECT id FROM incidents
            WHERE guild_id = $1 
            AND user_id = $2
            AND created_at >= NOW() - INTERVAL '5 minutes'
            AND action_taken IS NULL
            ORDER BY created_at DESC
            LIMIT 1
        )
        "#,
        guild_id,
        user_id,
        action
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn update_user_reputation(pool: &PgPool, user_id: i64, delta: i32) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE users
        SET global_reputation = global_reputation + $2
        WHERE user_id = $1
        "#,
        user_id,
        delta
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn remove_from_whitelist(pool: &PgPool, guild_id: i64, user_id: i64) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM whitelisted_users
        WHERE guild_id = $1 AND user_id = $2
        "#,
        guild_id,
        user_id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

pub async fn get_whitelisted_users(pool: &PgPool, guild_id: i64) -> Result<Vec<WhitelistedUser>> {
    let users = sqlx::query_as!(
        WhitelistedUser,
        r#"
        SELECT * FROM whitelisted_users
        WHERE guild_id = $1
        ORDER BY created_at DESC
        "#,
        guild_id
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}

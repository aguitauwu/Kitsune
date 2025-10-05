use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Guild {
    pub guild_id: i64,
    pub name: String,
    pub owner_id: i64,
    pub config: JsonValue,
    pub raid_threshold_5s: i32,
    pub raid_threshold_30s: i32,
    pub raid_threshold_1m: i32,
    pub raid_threshold_5m: i32,
    pub new_account_days: i32,
    pub auto_mod_enabled: bool,
    pub lockdown_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub discriminator: Option<String>,
    pub global_reputation: i32,
    pub total_incidents: i32,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BehaviorProfile {
    pub id: i32,
    pub guild_id: i64,
    pub user_id: i64,
    pub message_count: i32,
    pub join_timestamp: Option<DateTime<Utc>>,
    pub last_message_time: Option<DateTime<Utc>>,
    pub spam_score: f32,
    pub link_density: f32,
    pub mention_ratio: f32,
    pub caps_ratio: f32,
    pub emoji_density: f32,
    pub channel_diversity: f32,
    pub reply_ratio: f32,
    pub unique_interactions: i32,
    pub features: JsonValue,
    pub threat_score: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Incident {
    pub id: Uuid,
    pub guild_id: i64,
    pub user_id: i64,
    pub incident_type: String,
    pub severity: String,
    pub threat_score: f32,
    pub evidence: JsonValue,
    pub action_taken: Option<String>,
    pub moderator_id: Option<i64>,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ForensicEvent {
    pub id: Uuid,
    pub guild_id: i64,
    pub user_id: Option<i64>,
    pub event_type: String,
    pub content: Option<String>,
    pub metadata: JsonValue,
    pub threat_score: f32,
    pub related_events: Option<Vec<Uuid>>,
    pub tags: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct HoneypotCatch {
    pub id: Uuid,
    pub guild_id: i64,
    pub user_id: i64,
    pub trap_type: String,
    pub trap_name: String,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WhitelistedUser {
    pub id: i32,
    pub guild_id: i64,
    pub user_id: i64,
    pub reason: Option<String>,
    pub added_by: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl ThreatLevel {
    pub fn from_score(score: f32) -> Self {
        if score >= 0.95 {
            ThreatLevel::Critical
        } else if score >= 0.8 {
            ThreatLevel::High
        } else if score >= 0.6 {
            ThreatLevel::Medium
        } else {
            ThreatLevel::Low
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ThreatLevel::Low => "Low",
            ThreatLevel::Medium => "Medium",
            ThreatLevel::High => "High",
            ThreatLevel::Critical => "Critical",
        }
    }

    pub fn color(&self) -> u32 {
        match self {
            ThreatLevel::Low => 0x3498db,
            ThreatLevel::Medium => 0xf39c12,
            ThreatLevel::High => 0xe67e22,
            ThreatLevel::Critical => 0xe74c3c,
        }
    }
}

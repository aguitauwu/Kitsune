pub mod raid_detector;
pub mod behavior_analyzer;
pub mod honeypot;
pub mod auto_mod;
pub mod threat_calculator;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinEvent {
    pub user_id: i64,
    pub username: String,
    pub discriminator: Option<String>,
    pub account_created: DateTime<Utc>,
    pub join_time: DateTime<Utc>,
    pub avatar_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAnalysis {
    pub has_links: bool,
    pub link_count: usize,
    pub mention_count: usize,
    pub caps_ratio: f32,
    pub emoji_count: usize,
    pub text_similarity: f32,
    pub is_burst: bool,
}

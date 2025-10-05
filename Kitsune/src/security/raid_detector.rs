use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use strsim::jaro_winkler;

use super::JoinEvent;
use crate::config::SecurityConfig;

pub struct RaidDetector {
    config: SecurityConfig,
    join_events: Arc<DashMap<i64, Vec<JoinEvent>>>,
}

#[derive(Debug, Clone)]
pub struct RaidAnalysis {
    pub is_raid: bool,
    pub threat_score: f32,
    pub join_rate_5s: u32,
    pub join_rate_30s: u32,
    pub join_rate_1m: u32,
    pub join_rate_5m: u32,
    pub new_account_ratio: f32,
    pub username_similarity: f32,
    pub avatar_duplication: f32,
    pub reasons: Vec<String>,
}

impl RaidDetector {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            join_events: Arc::new(DashMap::new()),
        }
    }

    pub fn record_join(&self, guild_id: i64, event: JoinEvent) {
        let mut events = self.join_events.entry(guild_id).or_insert_with(Vec::new);
        events.push(event.clone());
        
        self.cleanup_old_events(guild_id);
    }

    pub fn analyze_raid_risk(&self, guild_id: i64) -> RaidAnalysis {
        let events = match self.join_events.get(&guild_id) {
            Some(e) => e,
            None => return RaidAnalysis::safe(),
        };

        let now = Utc::now();
        let mut reasons = Vec::new();

        let join_rate_5s = self.count_joins_in_window(&events, now, Duration::seconds(5));
        let join_rate_30s = self.count_joins_in_window(&events, now, Duration::seconds(30));
        let join_rate_1m = self.count_joins_in_window(&events, now, Duration::minutes(1));
        let join_rate_5m = self.count_joins_in_window(&events, now, Duration::minutes(5));

        let new_account_ratio = self.calculate_new_account_ratio(&events, now);
        let username_similarity = self.calculate_username_similarity(&events);
        let avatar_duplication = self.calculate_avatar_duplication(&events);

        let mut threat_score = 0.0f32;

        if join_rate_5s >= self.config.raid_threshold_5s {
            threat_score += 0.3;
            reasons.push(format!("{} joins in 5 seconds", join_rate_5s));
        }

        if join_rate_30s >= self.config.raid_threshold_30s {
            threat_score += 0.25;
            reasons.push(format!("{} joins in 30 seconds", join_rate_30s));
        }

        if join_rate_1m >= self.config.raid_threshold_1m {
            threat_score += 0.2;
            reasons.push(format!("{} joins in 1 minute", join_rate_1m));
        }

        if new_account_ratio > 0.7 {
            threat_score += 0.25;
            reasons.push(format!("{:.0}% new accounts", new_account_ratio * 100.0));
        }

        if username_similarity > self.config.username_similarity_threshold as f32 {
            threat_score += 0.2;
            reasons.push(format!("High username similarity ({:.2})", username_similarity));
        }

        if avatar_duplication > 0.5 {
            threat_score += 0.15;
            reasons.push(format!("{:.0}% duplicate avatars", avatar_duplication * 100.0));
        }

        let is_raid = threat_score >= 0.6;

        RaidAnalysis {
            is_raid,
            threat_score: threat_score.min(1.0),
            join_rate_5s,
            join_rate_30s,
            join_rate_1m,
            join_rate_5m,
            new_account_ratio,
            username_similarity,
            avatar_duplication,
            reasons,
        }
    }

    fn count_joins_in_window(
        &self,
        events: &[JoinEvent],
        now: DateTime<Utc>,
        window: Duration,
    ) -> u32 {
        let cutoff = now - window;
        events
            .iter()
            .filter(|e| e.join_time >= cutoff)
            .count() as u32
    }

    fn calculate_new_account_ratio(&self, events: &[JoinEvent], now: DateTime<Utc>) -> f32 {
        if events.is_empty() {
            return 0.0;
        }

        let cutoff = now - Duration::minutes(1);
        let recent_joins: Vec<_> = events.iter().filter(|e| e.join_time >= cutoff).collect();

        if recent_joins.is_empty() {
            return 0.0;
        }

        let new_account_threshold = Duration::days(self.config.new_account_days as i64);
        let new_accounts = recent_joins
            .iter()
            .filter(|e| {
                let account_age = now.signed_duration_since(e.account_created);
                account_age < new_account_threshold
            })
            .count();

        new_accounts as f32 / recent_joins.len() as f32
    }

    fn calculate_username_similarity(&self, events: &[JoinEvent]) -> f32 {
        if events.len() < 2 {
            return 0.0;
        }

        let cutoff = Utc::now() - Duration::minutes(1);
        let recent: Vec<_> = events.iter().filter(|e| e.join_time >= cutoff).collect();

        if recent.len() < 2 {
            return 0.0;
        }

        let mut similarities = Vec::new();
        for i in 0..recent.len() {
            for j in (i + 1)..recent.len() {
                let sim = jaro_winkler(&recent[i].username, &recent[j].username);
                similarities.push(sim);
            }
        }

        if similarities.is_empty() {
            0.0
        } else {
            similarities.iter().sum::<f64>() as f32 / similarities.len() as f32
        }
    }

    fn calculate_avatar_duplication(&self, events: &[JoinEvent]) -> f32 {
        if events.is_empty() {
            return 0.0;
        }

        let cutoff = Utc::now() - Duration::minutes(1);
        let recent: Vec<_> = events.iter().filter(|e| e.join_time >= cutoff).collect();

        if recent.len() < 2 {
            return 0.0;
        }

        let mut avatar_counts = std::collections::HashMap::new();
        for event in &recent {
            if let Some(hash) = &event.avatar_hash {
                *avatar_counts.entry(hash.clone()).or_insert(0) += 1;
            }
        }

        let max_duplicates = avatar_counts.values().max().copied().unwrap_or(0);
        max_duplicates as f32 / recent.len() as f32
    }

    fn cleanup_old_events(&self, guild_id: i64) {
        if let Some(mut events) = self.join_events.get_mut(&guild_id) {
            let cutoff = Utc::now() - Duration::minutes(10);
            events.retain(|e| e.join_time >= cutoff);
        }
    }
}

impl RaidAnalysis {
    fn safe() -> Self {
        Self {
            is_raid: false,
            threat_score: 0.0,
            join_rate_5s: 0,
            join_rate_30s: 0,
            join_rate_1m: 0,
            join_rate_5m: 0,
            new_account_ratio: 0.0,
            username_similarity: 0.0,
            avatar_duplication: 0.0,
            reasons: Vec::new(),
        }
    }
}

use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use strsim::jaro_winkler;

use super::MessageAnalysis;

const MAX_MESSAGE_HISTORY: usize = 100;

pub struct BehaviorAnalyzer {
    message_history: Arc<DashMap<(i64, i64), VecDeque<MessageRecord>>>,
}

#[derive(Debug, Clone)]
struct MessageRecord {
    content: String,
    timestamp: DateTime<Utc>,
    _channel_id: i64,
    has_links: bool,
    mention_count: usize,
}

#[derive(Debug, Clone)]
pub struct BehavioralMetrics {
    pub spam_score: f32,
    pub link_density: f32,
    pub mention_ratio: f32,
    pub caps_ratio: f32,
    pub emoji_density: f32,
    pub burst_detected: bool,
    pub threat_score: f32,
}

impl BehaviorAnalyzer {
    pub fn new() -> Self {
        Self {
            message_history: Arc::new(DashMap::new()),
        }
    }

    pub fn analyze_message(
        &self,
        guild_id: i64,
        user_id: i64,
        content: &str,
        channel_id: i64,
    ) -> MessageAnalysis {
        let has_links = content.contains("http://") || content.contains("https://");
        let link_count = content.matches("http").count();
        
        let mention_count = content.matches("<@").count();
        
        let total_chars = content.chars().count();
        let uppercase_chars = content.chars().filter(|c| c.is_uppercase()).count();
        let caps_ratio = if total_chars > 0 {
            uppercase_chars as f32 / total_chars as f32
        } else {
            0.0
        };
        
        let emoji_count = content.chars().filter(|c| {
            let code = *c as u32;
            (0x1F600..=0x1F64F).contains(&code) || 
            (0x1F300..=0x1F5FF).contains(&code) ||
            (0x1F680..=0x1F6FF).contains(&code)
        }).count();

        let record = MessageRecord {
            content: content.to_string(),
            timestamp: Utc::now(),
            _channel_id: channel_id,
            has_links,
            mention_count,
        };

        let mut history = self.message_history
            .entry((guild_id, user_id))
            .or_insert_with(VecDeque::new);
        
        history.push_back(record);
        if history.len() > MAX_MESSAGE_HISTORY {
            history.pop_front();
        }

        let text_similarity = self.calculate_text_similarity(&history);
        
        let is_burst = self.detect_burst(&history);

        MessageAnalysis {
            has_links,
            link_count,
            mention_count,
            caps_ratio,
            emoji_count,
            text_similarity,
            is_burst,
        }
    }

    pub fn get_behavioral_metrics(&self, guild_id: i64, user_id: i64) -> BehavioralMetrics {
        let history = match self.message_history.get(&(guild_id, user_id)) {
            Some(h) => h,
            None => return BehavioralMetrics::default(),
        };

        if history.is_empty() {
            return BehavioralMetrics::default();
        }

        let messages_with_links = history.iter().filter(|m| m.has_links).count();
        let link_density = messages_with_links as f32 / history.len() as f32;

        let total_mentions: usize = history.iter().map(|m| m.mention_count).sum();
        let mention_ratio = total_mentions as f32 / history.len() as f32;

        let spam_score = self.calculate_spam_score(&history);
        let caps_ratio = self.calculate_average_caps(&history);
        let emoji_density = self.calculate_emoji_density(&history);
        let burst_detected = self.detect_burst(&history);

        let mut threat_score: f32 = 0.0;

        if spam_score > 0.8 {
            threat_score += 0.3;
        } else if spam_score > 0.6 {
            threat_score += 0.15;
        }

        if link_density > 0.5 && history.len() < 10 {
            threat_score += 0.25;
        }

        if mention_ratio > 3.0 {
            threat_score += 0.2;
        }

        if caps_ratio > 0.7 {
            threat_score += 0.15;
        }

        if burst_detected {
            threat_score += 0.2;
        }

        BehavioralMetrics {
            spam_score,
            link_density,
            mention_ratio,
            caps_ratio,
            emoji_density,
            burst_detected,
            threat_score: threat_score.min(1.0),
        }
    }

    fn calculate_text_similarity(&self, history: &VecDeque<MessageRecord>) -> f32 {
        if history.len() < 2 {
            return 0.0;
        }

        let recent: Vec<_> = history.iter().rev().take(10).collect();
        if recent.len() < 2 {
            return 0.0;
        }

        let mut similarities = Vec::new();
        for i in 0..recent.len() {
            for j in (i + 1)..recent.len() {
                let sim = jaro_winkler(&recent[i].content, &recent[j].content);
                similarities.push(sim);
            }
        }

        if similarities.is_empty() {
            0.0
        } else {
            similarities.iter().sum::<f64>() as f32 / similarities.len() as f32
        }
    }

    fn calculate_spam_score(&self, history: &VecDeque<MessageRecord>) -> f32 {
        if history.len() < 3 {
            return 0.0;
        }

        let recent: Vec<_> = history.iter().rev().take(20).collect();
        let similarity = self.calculate_text_similarity(&history.iter().rev().take(20).cloned().collect());

        let mut score = similarity;

        let time_variance = self.calculate_time_variance(&recent);
        if time_variance < 2.0 {
            score += 0.2;
        }

        score.min(1.0)
    }

    fn calculate_time_variance(&self, messages: &[&MessageRecord]) -> f64 {
        if messages.len() < 2 {
            return 100.0;
        }

        let mut intervals = Vec::new();
        for i in 1..messages.len() {
            let interval = messages[i - 1]
                .timestamp
                .signed_duration_since(messages[i].timestamp)
                .num_seconds()
                .abs() as f64;
            intervals.push(interval);
        }

        if intervals.is_empty() {
            return 100.0;
        }

        let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / intervals.len() as f64;

        variance.sqrt()
    }

    fn calculate_average_caps(&self, history: &VecDeque<MessageRecord>) -> f32 {
        if history.is_empty() {
            return 0.0;
        }

        let total_chars: usize = history.iter().map(|m| m.content.chars().count()).sum();
        let uppercase_chars: usize = history
            .iter()
            .map(|m| m.content.chars().filter(|c| c.is_uppercase()).count())
            .sum();

        if total_chars > 0 {
            uppercase_chars as f32 / total_chars as f32
        } else {
            0.0
        }
    }

    fn calculate_emoji_density(&self, history: &VecDeque<MessageRecord>) -> f32 {
        if history.is_empty() {
            return 0.0;
        }

        let total_chars: usize = history.iter().map(|m| m.content.chars().count()).sum();
        let emoji_count: usize = history
            .iter()
            .map(|m| {
                m.content.chars().filter(|c| {
                    let code = *c as u32;
                    (0x1F600..=0x1F64F).contains(&code) || 
                    (0x1F300..=0x1F5FF).contains(&code) ||
                    (0x1F680..=0x1F6FF).contains(&code)
                }).count()
            })
            .sum();

        if total_chars > 0 {
            emoji_count as f32 / total_chars as f32
        } else {
            0.0
        }
    }

    fn detect_burst(&self, history: &VecDeque<MessageRecord>) -> bool {
        let cutoff = Utc::now() - Duration::seconds(10);
        let recent_count = history.iter().filter(|m| m.timestamp >= cutoff).count();
        recent_count >= 10
    }
}

impl Default for BehavioralMetrics {
    fn default() -> Self {
        Self {
            spam_score: 0.0,
            link_density: 0.0,
            mention_ratio: 0.0,
            caps_ratio: 0.0,
            emoji_density: 0.0,
            burst_detected: false,
            threat_score: 0.0,
        }
    }
}

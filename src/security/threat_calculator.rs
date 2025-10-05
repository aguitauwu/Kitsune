use super::raid_detector::RaidAnalysis;
use super::behavior_analyzer::BehavioralMetrics;

pub struct ThreatCalculator;

impl ThreatCalculator {
    pub fn calculate_combined_threat(
        raid_analysis: &RaidAnalysis,
        behavioral_metrics: &BehavioralMetrics,
        honeypot_multiplier: f32,
        is_new_account: bool,
    ) -> f32 {
        let mut base_score = 0.0;

        base_score += raid_analysis.threat_score * 0.4;
        
        base_score += behavioral_metrics.threat_score * 0.4;

        if honeypot_multiplier > 0.0 {
            base_score += honeypot_multiplier * 0.3;
        }

        if is_new_account {
            base_score += 0.1;
        }

        base_score.min(1.0)
    }

    #[allow(dead_code)]
    pub fn should_take_action(threat_score: f32, threshold: f32) -> bool {
        threat_score >= threshold
    }
}

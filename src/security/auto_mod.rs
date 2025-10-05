use crate::config::AutoModConfig;
use crate::database::models::ThreatLevel;

pub struct AutoModerator {
    config: AutoModConfig,
}

#[derive(Debug, Clone)]
pub enum ModAction {
    Monitor,
    Timeout { duration_minutes: u32 },
    Kick { reason: String },
    Ban { reason: String, delete_days: u8 },
    #[allow(dead_code)]
    Lockdown,
}

impl AutoModerator {
    pub fn new(config: AutoModConfig) -> Self {
        Self { config }
    }

    pub fn determine_action(&self, threat_score: f32, threat_level: ThreatLevel) -> Option<ModAction> {
        if !self.config.enabled {
            return None;
        }

        match threat_level {
            ThreatLevel::Low => {
                if threat_score >= self.config.low_threat_threshold {
                    Some(ModAction::Monitor)
                } else {
                    None
                }
            }
            ThreatLevel::Medium => {
                Some(ModAction::Timeout {
                    duration_minutes: 10,
                })
            }
            ThreatLevel::High => {
                Some(ModAction::Kick {
                    reason: format!("High threat score: {:.2}", threat_score),
                })
            }
            ThreatLevel::Critical => {
                Some(ModAction::Ban {
                    reason: format!("Critical threat detected: {:.2}", threat_score),
                    delete_days: 7,
                })
            }
        }
    }

    pub fn should_lockdown(&self, raid_threat_score: f32, recent_bans: u32) -> bool {
        raid_threat_score >= self.config.critical_threat_threshold && recent_bans >= 3
    }
}

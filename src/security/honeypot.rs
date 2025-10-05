use dashmap::DashMap;
use std::sync::Arc;

pub struct HoneypotSystem {
    hidden_channels: Arc<DashMap<i64, Vec<i64>>>,
    fake_commands: Arc<DashMap<i64, Vec<String>>>,
    catches: Arc<DashMap<(i64, i64), Vec<HoneypotCatch>>>,
}

#[derive(Debug, Clone)]
pub struct HoneypotCatch {
    pub trap_type: String,
    pub trap_name: String,
    pub severity: f32,
}

impl HoneypotSystem {
    pub fn new() -> Self {
        Self {
            hidden_channels: Arc::new(DashMap::new()),
            fake_commands: Arc::new(DashMap::new()),
            catches: Arc::new(DashMap::new()),
        }
    }

    #[allow(dead_code)]
    pub fn register_hidden_channel(&self, guild_id: i64, channel_id: i64) {
        let mut channels = self.hidden_channels.entry(guild_id).or_insert_with(Vec::new);
        if !channels.contains(&channel_id) {
            channels.push(channel_id);
        }
    }

    #[allow(dead_code)]
    pub fn register_fake_command(&self, guild_id: i64, command: String) {
        let mut commands = self.fake_commands.entry(guild_id).or_insert_with(Vec::new);
        if !commands.contains(&command) {
            commands.push(command);
        }
    }

    pub fn check_hidden_channel(&self, guild_id: i64, channel_id: i64, user_id: i64) -> bool {
        if let Some(channels) = self.hidden_channels.get(&guild_id) {
            if channels.contains(&channel_id) {
                self.record_catch(guild_id, user_id, HoneypotCatch {
                    trap_type: "hidden_channel".to_string(),
                    trap_name: format!("channel_{}", channel_id),
                    severity: 0.8,
                });
                return true;
            }
        }
        false
    }

    pub fn check_fake_command(&self, guild_id: i64, command: &str, user_id: i64) -> bool {
        if let Some(commands) = self.fake_commands.get(&guild_id) {
            let command_lower = command.to_lowercase();
            for fake_cmd in commands.iter() {
                if command_lower.starts_with(fake_cmd) {
                    self.record_catch(guild_id, user_id, HoneypotCatch {
                        trap_type: "fake_command".to_string(),
                        trap_name: fake_cmd.clone(),
                        severity: 0.7,
                    });
                    return true;
                }
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn check_suspicious_timing(&self, guild_id: i64, user_id: i64, reaction_time_ms: u64) -> bool {
        if reaction_time_ms < 100 {
            self.record_catch(guild_id, user_id, HoneypotCatch {
                trap_type: "suspicious_timing".to_string(),
                trap_name: format!("reaction_{}ms", reaction_time_ms),
                severity: 0.6,
            });
            return true;
        }
        false
    }

    fn record_catch(&self, guild_id: i64, user_id: i64, catch: HoneypotCatch) {
        let mut catches = self.catches.entry((guild_id, user_id)).or_insert_with(Vec::new);
        catches.push(catch);
    }

    pub fn get_user_catches(&self, guild_id: i64, user_id: i64) -> Vec<HoneypotCatch> {
        self.catches
            .get(&(guild_id, user_id))
            .map(|c| c.clone())
            .unwrap_or_default()
    }

    pub fn get_threat_multiplier(&self, guild_id: i64, user_id: i64) -> f32 {
        let catches = self.get_user_catches(guild_id, user_id);
        if catches.is_empty() {
            return 0.0;
        }

        let total_severity: f32 = catches.iter().map(|c| c.severity).sum();
        (total_severity / catches.len() as f32).min(1.0)
    }
}

impl Default for HoneypotSystem {
    fn default() -> Self {
        Self::new()
    }
}

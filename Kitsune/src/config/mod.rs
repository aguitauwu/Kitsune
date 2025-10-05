use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
    pub redis_url: String,
    pub security: SecurityConfig,
    pub auto_mod: AutoModConfig,
    pub forensics: ForensicsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub raid_threshold_5s: u32,
    pub raid_threshold_30s: u32,
    pub raid_threshold_1m: u32,
    pub raid_threshold_5m: u32,
    pub new_account_days: u32,
    pub username_similarity_threshold: f64,
    pub spam_similarity_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoModConfig {
    pub enabled: bool,
    pub low_threat_threshold: f32,
    pub medium_threat_threshold: f32,
    pub high_threat_threshold: f32,
    pub critical_threat_threshold: f32,
    pub message_burst_count: u32,
    pub message_burst_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicsConfig {
    pub retention_days: i64,
    pub detailed_logging: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            raid_threshold_5s: 5,
            raid_threshold_30s: 10,
            raid_threshold_1m: 15,
            raid_threshold_5m: 30,
            new_account_days: 7,
            username_similarity_threshold: 0.85,
            spam_similarity_threshold: 0.80,
        }
    }
}

impl Default for AutoModConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            low_threat_threshold: 0.3,
            medium_threat_threshold: 0.6,
            high_threat_threshold: 0.8,
            critical_threat_threshold: 0.95,
            message_burst_count: 10,
            message_burst_seconds: 10,
        }
    }
}

impl Default for ForensicsConfig {
    fn default() -> Self {
        Self {
            retention_days: 90,
            detailed_logging: true,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let discord_token = env::var("DISCORD_TOKEN")
            .context("DISCORD_TOKEN environment variable not set")?;
        
        let database_url = env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable not set")?;
        
        let redis_url = env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());

        if std::path::Path::new(&config_path).exists() {
            Self::from_file(&config_path, discord_token, database_url, redis_url)
        } else {
            Ok(Self::with_defaults(discord_token, database_url, redis_url))
        }
    }

    fn from_file(path: &str, discord_token: String, database_url: String, redis_url: String) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .context(format!("Failed to read config file: {}", path))?;
        
        let mut config: Self = toml::from_str(&contents)
            .context("Failed to parse config file")?;
        
        config.discord_token = discord_token;
        config.database_url = database_url;
        config.redis_url = redis_url;
        
        Ok(config)
    }

    fn with_defaults(discord_token: String, database_url: String, redis_url: String) -> Self {
        Self {
            discord_token,
            database_url,
            redis_url,
            security: SecurityConfig::default(),
            auto_mod: AutoModConfig::default(),
            forensics: ForensicsConfig::default(),
        }
    }
}

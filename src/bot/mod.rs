pub mod commands;
pub mod commands_extra;
pub mod events;

use anyhow::Result;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

use crate::config::Config;
use crate::security::{
    raid_detector::RaidDetector,
    behavior_analyzer::BehaviorAnalyzer,
    honeypot::HoneypotSystem,
    auto_mod::AutoModerator,
};

pub struct Data {
    pub pool: PgPool,
    pub _redis: ConnectionManager,
    pub config: Config,
    pub raid_detector: Arc<RaidDetector>,
    pub behavior_analyzer: Arc<BehaviorAnalyzer>,
    pub honeypot: Arc<HoneypotSystem>,
    pub auto_mod: Arc<AutoModerator>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn create_framework(config: Config, pool: PgPool, redis: ConnectionManager) -> Result<poise::Framework<Data, Error>> {
    let raid_detector = Arc::new(RaidDetector::new(config.security.clone()));
    let behavior_analyzer = Arc::new(BehaviorAnalyzer::new());
    let honeypot = Arc::new(HoneypotSystem::new());
    let auto_mod = Arc::new(AutoModerator::new(config.auto_mod.clone()));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::kitsune(),
                commands::help::help(),
                commands_extra::reputation(),
                commands_extra::access(),
                commands_extra::insights(),
                commands_extra::admin(),
                commands_extra::info(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                tracing::info!("All slash commands registered with Discord");
                
                Ok(Data {
                    pool,
                    _redis: redis,
                    config,
                    raid_detector,
                    behavior_analyzer,
                    honeypot,
                    auto_mod,
                })
            })
        })
        .build();

    Ok(framework)
}

use anyhow::Result;
use redis::{Client, aio::ConnectionManager};

pub async fn create_redis_client(redis_url: &str) -> Result<ConnectionManager> {
    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;
    Ok(manager)
}

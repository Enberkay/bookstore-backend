// Redis Connector
// Handles Redis connection and pool management

use deadpool_redis::{Config, Pool, Runtime};
use anyhow::{Context, Result};
use crate::{
    config::config_model::Redis,
};

// Redis connection pool wrapper
#[derive(Clone, Debug)]
pub struct RedisPool {
    pool: Pool,
}

impl RedisPool {
    // Create a new Redis pool from configuration
    pub async fn new(redis_config: &Redis) -> Result<Self> {
        let config = Config::from_url(&redis_config.url);
        let pool = config
            .create_pool(Some(Runtime::Tokio1))
            .context("Failed to create Redis pool")?;

        Ok(Self { pool })
    }

    // Get a connection from the pool
    pub async fn get_connection(&self) -> Result<deadpool_redis::Connection> {
        self.pool
            .get()
            .await
            .context("Failed to get Redis connection")
    }

    // Get the underlying pool (for direct access if needed)
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}

impl std::ops::Deref for RedisPool {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

use async_trait::async_trait;
use chrono::Utc;
use redis::{AsyncCommands, RedisResult};
use serde_json;

use anyhow::{Context, Result};
use crate::{
    domain::entities::refresh_token::{NewRefreshToken, RefreshTokenEntity},
    domain::repositories::token_repository::TokenRepository,
    infrastructure::redis::redis_connector::RedisPool,
};

#[derive(Clone, Debug)]
pub struct RedisTokenRepository {
    redis_pool: RedisPool,
    refresh_token_expiry_days: u64,
}

impl RedisTokenRepository {
    /// Create a new Redis token repository
    pub fn new(redis_pool: RedisPool, refresh_token_expiry_days: u64) -> Self {
        Self {
            redis_pool,
            refresh_token_expiry_days,
        }
    }

    /// Generate Redis key for refresh token
    fn refresh_token_key(&self, token_hash: &str) -> String {
        format!("refresh_token:{}", token_hash)
    }

    /// Generate Redis key for user sessions set
    fn user_sessions_key(&self, user_id: i32) -> String {
        format!("user_sessions:{}", user_id)
    }

    /// Calculate TTL in seconds from days
    fn calculate_ttl(&self) -> u64 {
        self.refresh_token_expiry_days * 24 * 60 * 60 // Convert days to seconds
    }
}

#[async_trait]
impl TokenRepository for RedisTokenRepository {
    async fn store_refresh_token(&self, token: NewRefreshToken) -> Result<()> {
        let mut conn = self.redis_pool.get_connection().await?;

        let token_hash = token.token_hash.clone();
        let token_key = self.refresh_token_key(&token_hash);
        let token_entity = RefreshTokenEntity::new(
            token.user_id,
            token_hash,
            token.expires_at,
            Utc::now(),
        );

        let token_data = serde_json::to_string(&token_entity)
            .context("Failed to serialize token")?;
        let user_sessions_key = self.user_sessions_key(token.user_id);
        let ttl = self.calculate_ttl();

        // Store token data with TTL
        let _: RedisResult<()> = conn.set_ex(&token_key, &token_data, ttl).await;

        // Add token hash to user sessions set
        let _: RedisResult<()> = conn.sadd(&user_sessions_key, &token.token_hash).await;

        Ok(())
    }

    async fn get_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenEntity>> {
        let mut conn = self.redis_pool.get_connection().await?;
        let token_key = self.refresh_token_key(token_hash);

        let token_data: Option<String> = conn.get(token_key).await
            .context("Failed to get token from Redis")?;

        match token_data {
            Some(data) => {
                let token_entity: RefreshTokenEntity = serde_json::from_str(&data)
                    .context("Failed to deserialize token")?;
                Ok(Some(token_entity))
            }
            None => Ok(None),
        }
    }

    async fn revoke_token(&self, token_hash: &str) -> Result<()> {
        let mut conn = self.redis_pool.get_connection().await?;
        let token_key = self.refresh_token_key(token_hash);

        // Get token to find user_id
        let token_data: Option<String> = conn.get(&token_key).await
            .context("Failed to get token from Redis")?;

        if let Some(data) = token_data {
            let token_entity: RefreshTokenEntity = serde_json::from_str(&data)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize token: {}", e))?;

            let user_sessions_key = self.user_sessions_key(token_entity.user_id);

            // Remove token from user sessions set
            let _: RedisResult<()> = conn.srem(&user_sessions_key, token_hash).await;
        }

        // Delete token
        let _: RedisResult<()> = conn.del(&token_key).await;

        Ok(())
    }

}

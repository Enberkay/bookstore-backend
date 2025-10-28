use crate::domain::entities::refresh_token::{NewRefreshToken, RefreshTokenEntity};
use anyhow::Result;

#[async_trait::async_trait]
pub trait JwtRepository: Send + Sync {
    // Create an access token for the given user with roles and permissions
    async fn create_access_token(
        &self,
        user_id: i32,
        roles: &[String],
        permissions: &[String],
    ) -> Result<String>;

    // Validate an access token and return the user ID
    async fn validate_access_token(&self, token: &str) -> Result<i32>;

    // Create a refresh token for the given user
    async fn create_refresh_token(&self, user_id: i32, expiry_days: u64) -> Result<String>;

    // Validate a refresh token and return the user ID
    async fn validate_refresh_token(&self, token: &str) -> Result<i32>;

    // Hash a refresh token for storage
    async fn hash_refresh_token(&self, token: &str) -> Result<String>;
}

#[async_trait::async_trait]
pub trait TokenRepository: Send + Sync {
    // Store a new refresh token
    async fn store_refresh_token(&self, token: NewRefreshToken) -> Result<()>;

    // Get refresh token by hash
    async fn get_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenEntity>>;

    // Revoke a refresh token by hash
    async fn revoke_token(&self, token_hash: &str) -> Result<()>;
}

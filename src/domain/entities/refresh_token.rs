use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenEntity {
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshTokenEntity {
    pub fn new(
        user_id: i32,
        token_hash: String,
        expires_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            token_hash,
            expires_at,
            created_at,
        }
    }

    // Check if the token is expired
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        self.expires_at < now
    }

    // Check if token is valid (not expired)
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRefreshToken {
    pub user_id: i32,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
}

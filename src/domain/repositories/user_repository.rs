use async_trait::async_trait;

use crate::domain::entities::user::UserEntity;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, anyhow::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, anyhow::Error>;
    async fn find_all(&self) -> Result<Vec<UserEntity>, anyhow::Error>;
    async fn save(&self, user: &UserEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

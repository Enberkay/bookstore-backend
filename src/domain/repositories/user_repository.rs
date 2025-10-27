use crate::domain::entities::user::UserEntity;

pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, anyhow::Error>;
    fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, anyhow::Error>;
    fn find_all(&self) -> Result<Vec<UserEntity>, anyhow::Error>;
    fn save(&self, user: &UserEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

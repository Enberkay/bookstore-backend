use crate::domain::entities::role::RoleEntity;

pub trait RoleRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<RoleEntity>, anyhow::Error>;
    fn save(&self, role: &RoleEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

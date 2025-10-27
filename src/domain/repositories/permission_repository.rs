use crate::domain::entities::permission::PermissionEntity;

pub trait PermissionRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<PermissionEntity>, anyhow::Error>;
    fn save(&self, permission: &PermissionEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

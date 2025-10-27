use crate::domain::entities::branch::BranchEntity;

pub trait BranchRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<BranchEntity>, anyhow::Error>;
    fn find_all(&self) -> Result<Vec<BranchEntity>, anyhow::Error>;
    fn save(&self, branch: &BranchEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

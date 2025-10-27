use crate::domain::entities::category::CategoryEntity;

pub trait CategoryRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<CategoryEntity>, anyhow::Error>;
    fn find_all(&self) -> Result<Vec<CategoryEntity>, anyhow::Error>;
    fn save(&self, category: &CategoryEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

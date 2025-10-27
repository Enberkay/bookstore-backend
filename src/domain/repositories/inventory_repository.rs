use crate::domain::entities::inventory::InventoryEntity;
use crate::domain::value_objects::isbn13::Isbn13;

pub trait InventoryRepository: Send + Sync {
    fn find(&self, branch_id: i32, isbn: &Isbn13)-> Result<Option<InventoryEntity>, anyhow::Error>;
    fn save(&self, inventory: &InventoryEntity) -> Result<(), anyhow::Error>;
    fn update(&self, inventory: &InventoryEntity) -> Result<(), anyhow::Error>;
}

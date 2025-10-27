use crate::domain::entities::sale::{SaleEntity, SaleItemEntity};
use anyhow::Result;

pub trait SaleRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<SaleEntity>, anyhow::Error>;
    fn find_items(&self, sale_id: i32) -> Result<Vec<SaleItemEntity>, anyhow::Error>;
    fn save(&self, sale: &SaleEntity) -> Result<(), anyhow::Error>;
    fn save_items(&self, items: &[SaleItemEntity]) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<()>;
    fn save_with_items(&self, sale: &SaleEntity, items: &[SaleItemEntity]) -> Result<()> {
        self.save(sale)?;
        self.save_items(items)?;
        Ok(())
    }
}

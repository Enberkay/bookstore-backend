use crate::domain::entities::order::{OrderEntity, OrderItemEntity};
use anyhow::Result;

pub trait OrderRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Result<Option<OrderEntity>, anyhow::Error>;
    fn find_items(&self, order_id: i32) -> Result<Vec<OrderItemEntity>, anyhow::Error>;
    fn save(&self, order: &OrderEntity) -> Result<(), anyhow::Error>;
    fn save_items(&self, items: &[OrderItemEntity]) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<()>;
    fn save_with_items(&self, order: &OrderEntity, items: &[OrderItemEntity]) -> Result<()> {
            self.save(order)?;
            self.save_items(items)?;
            Ok(())
        }
}

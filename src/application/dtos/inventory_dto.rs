use serde::{Serialize, Deserialize};
use crate::domain::entities::inventory::InventoryEntity;

#[derive(Debug, Deserialize)]
pub struct CreateInventoryRequest {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryRequest {
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: u32,
    pub updated_at: String,
}

impl From<InventoryEntity> for InventoryResponse {
    fn from(entity: InventoryEntity) -> Self {
        Self {
            branch_id: entity.branch_id,
            book_isbn: entity.book_isbn.as_str().to_string(),
            quantity: entity.quantity.value(),
            updated_at: entity.updated_at.to_rfc3339(),
        }
    }
}

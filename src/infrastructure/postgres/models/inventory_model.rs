use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{
    infrastructure::postgres::schema::inventories,
    domain::{
        entities::inventory::InventoryEntity,
        value_objects::{isbn13::Isbn13, stock_quantity::StockQuantity},
    },
};

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = inventories)]
#[diesel(primary_key(branch_id, book_isbn))]
pub struct InventoryModel {
    pub branch_id: i32,
    pub book_isbn: String,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<InventoryModel> for InventoryEntity {
    fn from(model: InventoryModel) -> Self {
        Self {
            branch_id: model.branch_id,
            book_isbn: Isbn13::new(&model.book_isbn).expect("Invalid ISBN-13"),
            quantity: StockQuantity::new(model.quantity).expect("Invalid stock quantity"),
            updated_at: model.updated_at,
        }
    }
}

impl From<InventoryEntity> for InventoryModel {
    fn from(entity: InventoryEntity) -> Self {
        Self {
            branch_id: entity.branch_id,
            book_isbn: entity.book_isbn.to_string(),
            quantity: entity.quantity.value() as i32,
            updated_at: entity.updated_at,
        }
    }
}

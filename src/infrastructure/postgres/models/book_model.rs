use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

use crate::domain::{
    entities::book::BookEntity,
    value_objects::{isbn13::Isbn13, money::Money},
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BookModel {
    pub isbn: String,
    pub title: String,
    pub author: Option<String>,
    pub synopsis: Option<String>,
    pub price: Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// =============================
// Mapping between Model ↔ Entity
// =============================

impl From<BookModel> for BookEntity {
    fn from(model: BookModel) -> Self {
        Self {
            isbn: Isbn13::new(&model.isbn).expect("Invalid ISBN-13"),
            title: model.title,
            author: model.author,
            synopsis: model.synopsis,
            price: Money::new(model.price).expect("Invalid price"),
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<BookEntity> for BookModel {
    fn from(entity: BookEntity) -> Self {
        Self {
            isbn: entity.isbn.to_string(),
            title: entity.title,
            author: entity.author,
            synopsis: entity.synopsis,
            price: entity.price.value(),
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

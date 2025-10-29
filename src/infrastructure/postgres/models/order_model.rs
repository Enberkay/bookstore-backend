use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

use crate::domain::{
    entities::order::{OrderEntity, OrderItemEntity},
    value_objects::money::Money,
};

// ======================
// OrderModel
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderModel {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub source: String,
    pub total_amount: Decimal,
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<OrderModel> for OrderEntity {
    fn from(model: OrderModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            order_date: model.order_date,
            status: model.status,
            source: model.source,
            total_amount: Money::from_decimal(model.total_amount)
                .expect("Invalid total amount"),
            shipping_address: model.shipping_address,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<OrderEntity> for OrderModel {
    fn from(entity: OrderEntity) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user_id,
            order_date: entity.order_date,
            status: entity.status,
            source: entity.source,
            total_amount: entity.total_amount.to_decimal(),
            shipping_address: entity.shipping_address,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

// ======================
// OrderItemModel
// ======================

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItemModel {
    pub id: i32,
    pub order_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_purchase: Decimal,
    pub subtotal: Decimal,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<OrderItemModel> for OrderItemEntity {
    fn from(model: OrderItemModel) -> Self {
        Self {
            id: model.id,
            order_id: model.order_id,
            book_isbn: model.book_isbn,
            book_title: model.book_title,
            book_author: model.book_author,
            quantity: model.quantity,
            price_at_purchase: Money::from_decimal(model.price_at_purchase)
                .expect("Invalid price"),
            subtotal: Money::from_decimal(model.subtotal)
                .expect("Invalid subtotal"),
            created_at: model.created_at,
        }
    }
}

impl From<OrderItemEntity> for OrderItemModel {
    fn from(entity: OrderItemEntity) -> Self {
        Self {
            id: entity.id,
            order_id: entity.order_id,
            book_isbn: entity.book_isbn,
            book_title: entity.book_title,
            book_author: entity.book_author,
            quantity: entity.quantity,
            price_at_purchase: entity.price_at_purchase.to_decimal(),
            subtotal: entity.subtotal.to_decimal(),
            created_at: entity.created_at,
        }
    }
}

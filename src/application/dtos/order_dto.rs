use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::domain::entities::order::{OrderEntity, OrderItemEntity};

#[derive(Debug, Deserialize)]
pub struct CreateOrderItemRequest {
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i32,
    pub source: String, // "ONLINE" | "POS"
    pub shipping_address: Option<String>,
    pub items: Vec<CreateOrderItemRequest>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_purchase: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub source: String,
    pub total_amount: f64,
    pub shipping_address: Option<String>,
    pub items: Vec<OrderItemResponse>,
}

impl From<(OrderEntity, Vec<OrderItemEntity>)> for OrderResponse {
    fn from((order, items): (OrderEntity, Vec<OrderItemEntity>)) -> Self {
        Self {
            id: order.id,
            user_id: order.user_id,
            order_date: order.order_date,
            status: order.status,
            source: order.source,
            total_amount: order.total_amount.value().to_f64().unwrap_or(0.0),
            shipping_address: order.shipping_address,
            items: items.into_iter().map(|i| OrderItemResponse {
                book_isbn: i.book_isbn,
                book_title: i.book_title,
                book_author: i.book_author,
                quantity: i.quantity,
                price_at_purchase: i.price_at_purchase.value().to_f64().unwrap_or(0.0),
                subtotal: i.subtotal.value().to_f64().unwrap_or(0.0),
            }).collect(),
        }
    }
}

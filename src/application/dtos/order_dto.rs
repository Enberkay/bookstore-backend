use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: i32,
    pub total_amount: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i32,
    pub items: Vec<OrderItemRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderItemRequest {
    pub book_isbn: String,
    pub quantity: i32,
    pub price_at_purchase: f64,
}

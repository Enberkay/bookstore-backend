use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub amount: f64,
    pub method: String,
}

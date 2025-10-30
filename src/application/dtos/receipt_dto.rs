use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ReceiptResponse {
    pub id: i32,
    pub receipt_code: String,
    pub reference_id: i32,
    pub source: String,
    pub total_amount: f64,
    pub payment_method: Option<String>,
    pub status: String,
    pub issued_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReceiptRequest {
    pub reference_id: i32,
    pub source: String,
    pub total_amount: f64,
    pub payment_method: Option<String>,
}

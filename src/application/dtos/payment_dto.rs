use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::domain::entities::payment::PaymentEntity;

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub method: String,          // maps to payment_method
    pub amount: f64,
    pub reference: Option<String>, // maps to transaction_ref
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub payment_method: String,
    pub transaction_ref: Option<String>,
    pub amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<PaymentEntity> for PaymentResponse {
    fn from(entity: PaymentEntity) -> Self {
        Self {
            id: entity.id,
            order_id: entity.order_id,
            sale_id: entity.sale_id,
            payment_method: entity.payment_method,
            transaction_ref: entity.transaction_ref,
            amount: entity.amount.value().to_f64().unwrap_or(0.0),
            status: entity.status,
            created_at: entity.created_at,
        }
    }
}

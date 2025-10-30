use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;

use crate::domain::{
    entities::{sale::SaleEntity, payment::PaymentEntity, receipt::ReceiptEntity},
};


#[derive(Debug, Deserialize)]
pub struct CreateSaleRequest {
    pub user_id: i32,
    pub branch_id: i32,
    pub payment_method: String,
    pub total_amount: f64,
    pub items: Vec<SaleItemRequest>,
}

#[derive(Debug, Deserialize)]
pub struct SaleItemRequest {
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub unit_price: f64,
}


#[derive(Debug, Serialize)]
pub struct SaleResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: f64,
    pub payment_method: String,
    pub payment_ref: Option<String>,
    pub receipt_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<(SaleEntity, PaymentEntity, ReceiptEntity)> for SaleResponse {
    fn from((sale, payment, receipt): (SaleEntity, PaymentEntity, ReceiptEntity)) -> Self {
        Self {
            id: sale.id,
            user_id: sale.employee_id,
            branch_id: sale.branch_id,
            total_amount: sale.total_amount.value().to_f64().unwrap_or(0.0),
            payment_method: sale.payment_method.clone(),
            payment_ref: payment.transaction_ref.clone(),
            receipt_code: Some(receipt.receipt_code.as_str().to_string()),
            created_at: sale.created_at,
        }
    }
}

impl SaleResponse {
    /// Compose response when data comes from repository joins
    pub fn compose(
        sale: SaleEntity,
        payment: Option<PaymentEntity>,
        receipt: Option<ReceiptEntity>,
    ) -> Self {
        Self {
            id: sale.id,
            user_id: sale.employee_id,
            branch_id: sale.branch_id,
            total_amount: sale.total_amount.value().to_f64().unwrap_or(0.0),
            payment_method: sale.payment_method.clone(),
            payment_ref: payment.and_then(|p| p.transaction_ref),
            receipt_code: receipt.map(|r| r.receipt_code.as_str().to_string()),
            created_at: sale.created_at,
        }
    }
}

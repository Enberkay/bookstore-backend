use crate::domain::value_objects::{money::Money, receipt_code::ReceiptCode};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ReceiptEntity {
    pub id: i32,
    pub receipt_code: ReceiptCode,
    pub type_name: String, // "SALE" or "ORDER"
    pub reference_id: i32, // FK: sales.id or orders.id
    pub source: String,    // "POS" or "ONLINE"
    pub user_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: Money,
    pub payment_method: Option<String>,
    pub payment_ref: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub status: String, // e.g. "PAID", "CANCELLED"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReceiptEntity {
    pub fn new(
        receipt_code: ReceiptCode,
        type_name: String,
        reference_id: i32,
        source: String,
        total_amount: Money,
        user_id: Option<i32>,
        branch_id: Option<i32>,
        payment_method: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            receipt_code,
            type_name,
            reference_id,
            source,
            user_id,
            branch_id,
            total_amount,
            payment_method,
            payment_ref: None,
            issued_at: now,
            status: "PAID".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_cancelled(&mut self) {
        self.status = "CANCELLED".to_string();
        self.updated_at = Utc::now();
    }

    pub fn mark_paid(&mut self) {
        self.status = "PAID".to_string();
        self.updated_at = Utc::now();
    }

    pub fn summary(&self) -> String {
        format!(
            "{} - {} - {:.2} [{}]",
            self.receipt_code.as_str(),
            self.type_name,
            self.total_amount.value(),
            self.status
        )
    }
}

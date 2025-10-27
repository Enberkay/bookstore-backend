use crate::domain::value_objects::money::Money;
use chrono::{DateTime, Utc};

/// Represents a payment transaction, either from an Order or a Sale.
#[derive(Debug, Clone)]
pub struct PaymentEntity {
    pub id: i32,
    pub order_id: Option<i32>,
    pub sale_id: Option<i32>,
    pub payment_method: String, // e.g. "CASH", "CREDIT_CARD", "PROMPTPAY"
    pub transaction_ref: Option<String>,
    pub amount: Money,
    pub status: String, // e.g. "PENDING", "PAID", "FAILED", "REFUNDED"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PaymentEntity {
    /// Creates a new payment in `PENDING` status
    pub fn new(
        order_id: Option<i32>,
        sale_id: Option<i32>,
        payment_method: String,
        amount: Money,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // usually set by DB
            order_id,
            sale_id,
            payment_method,
            transaction_ref: None,
            amount,
            status: "PENDING".to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Marks this payment as successful
    pub fn mark_paid(&mut self, transaction_ref: Option<String>) {
        self.status = "PAID".to_string();
        self.transaction_ref = transaction_ref;
        self.updated_at = Utc::now();
    }

    /// Marks this payment as failed (e.g. declined)
    pub fn mark_failed(&mut self) {
        self.status = "FAILED".to_string();
        self.updated_at = Utc::now();
    }

    /// Marks this payment as refunded
    pub fn mark_refunded(&mut self) {
        self.status = "REFUNDED".to_string();
        self.updated_at = Utc::now();
    }

    /// Checks if payment is already settled
    pub fn is_paid(&self) -> bool {
        self.status.eq_ignore_ascii_case("PAID")
    }

    /// Checks if payment failed
    pub fn is_failed(&self) -> bool {
        self.status.eq_ignore_ascii_case("FAILED")
    }

    /// Validate the payment amount (domain invariant)
    pub fn validate_amount(&self) -> Result<(), String> {
        if self.amount.value() <= 0.0 {
            return Err("Payment amount must be greater than zero".into());
        }
        Ok(())
    }

    /// Returns a short human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "[{}] {} {:.2} ({})",
            self.status,
            self.payment_method,
            self.amount.value(),
            self.transaction_ref
                .clone()
                .unwrap_or_else(|| "no-ref".into())
        )
    }
}

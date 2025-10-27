use crate::domain::value_objects::money::Money;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct OrderEntity {
    pub id: i32,
    pub user_id: Option<i32>,
    pub order_date: DateTime<Utc>,
    pub status: String, // e.g. PENDING, PAID, SHIPPED
    pub source: String, // e.g. ONLINE
    pub total_amount: Money,
    pub shipping_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OrderEntity {
    /// Create new Order (usually from checkout)
    pub fn new(user_id: Option<i32>, source: String, shipping_address: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // temporary until persistence layer assigns
            user_id,
            order_date: now,
            status: "PENDING".into(),
            source,
            total_amount: Money::zero(),
            shipping_address,
            created_at: now,
            updated_at: now,
        }
    }

    /// Mark order as paid
    pub fn mark_paid(&mut self) {
        self.status = "PAID".into();
        self.updated_at = Utc::now();
    }

    /// Mark order as shipped
    pub fn mark_shipped(&mut self) {
        self.status = "SHIPPED".into();
        self.updated_at = Utc::now();
    }

    /// Cancel order (only if not shipped yet)
    pub fn cancel(&mut self) -> Result<(), String> {
        if self.status == "SHIPPED" {
            return Err("Cannot cancel an order that has already been shipped".into());
        }
        self.status = "CANCELLED".into();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update total amount
    pub fn update_total(&mut self, new_total: Money) {
        self.total_amount = new_total;
        self.updated_at = Utc::now();
    }

    /// Check if order is already paid
    pub fn is_paid(&self) -> bool {
        self.status == "PAID"
    }

    /// Check if order can be shipped
    pub fn can_ship(&self) -> bool {
        self.status == "PAID"
    }

    /// Check if order is cancellable
    pub fn can_cancel(&self) -> bool {
        self.status == "PENDING"
    }
}

#[derive(Debug, Clone)]
pub struct OrderItemEntity {
    pub id: i32,
    pub order_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_purchase: Money,
    pub subtotal: Money,
    pub created_at: DateTime<Utc>,
}

impl OrderItemEntity {
    /// Create new order item
    pub fn new(
        order_id: i32,
        book_isbn: String,
        book_title: String,
        book_author: Option<String>,
        quantity: i32,
        price_at_purchase: Money,
    ) -> Self {
        let subtotal = price_at_purchase * quantity as f64;
        Self {
            id: 0,
            order_id,
            book_isbn,
            book_title,
            book_author,
            quantity,
            price_at_purchase,
            subtotal,
            created_at: Utc::now(),
        }
    }

    /// Update quantity and recalculate subtotal
    pub fn update_quantity(&mut self, qty: i32) -> Result<(), String> {
        if qty <= 0 {
            return Err("Quantity must be greater than zero".into());
        }
        self.quantity = qty;
        self.subtotal = self.price_at_purchase * qty as f64;
        Ok(())
    }

    /// Calculate subtotal explicitly (safety)
    pub fn recalculate_subtotal(&mut self) {
        self.subtotal = self.price_at_purchase * self.quantity as f64;
    }
}

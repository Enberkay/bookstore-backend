use crate::domain::value_objects::money::Money;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SaleEntity {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub sale_date: DateTime<Utc>,
    pub total_amount: Money,
    pub payment_method: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SaleItemEntity {
    pub id: i32,
    pub sale_id: i32,
    pub book_isbn: String,
    pub book_title: String,
    pub book_author: Option<String>,
    pub quantity: i32,
    pub price_at_sale: Money,
    pub subtotal: Money,
    pub created_at: DateTime<Utc>,
}

impl SaleEntity {
    pub fn new(employee_id: Option<i32>, branch_id: Option<i32>, payment_method: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            employee_id,
            branch_id,
            sale_date: now,
            total_amount: Money::zero(),
            payment_method,
            created_at: now,
        }
    }

    pub fn update_total(&mut self, new_total: Money) {
        self.total_amount = new_total;
    }

    pub fn add_item(&mut self, item_total: Money) {
        self.total_amount = self.total_amount.add(item_total);
    }
}

impl SaleItemEntity {
    pub fn new(
        sale_id: i32,
        book_isbn: String,
        book_title: String,
        book_author: Option<String>,
        quantity: i32,
        price_at_sale: Money,
    ) -> Self {
        let subtotal = price_at_sale.multiply(quantity as u32);
        Self {
            id: 0,
            sale_id,
            book_isbn,
            book_title,
            book_author,
            quantity,
            price_at_sale,
            subtotal,
            created_at: Utc::now(),
        }
    }

    pub fn update_quantity(&mut self, new_qty: i32) {
        self.quantity = new_qty;
        self.subtotal = self.price_at_sale.multiply(new_qty as u32);
    }
}

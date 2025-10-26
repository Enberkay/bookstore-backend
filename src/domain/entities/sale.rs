use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SaleEntity {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub sale_date: DateTime<Utc>,
    pub total_amount: f64,
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
    pub price_at_sale: f64,
    pub subtotal: f64,
    pub created_at: DateTime<Utc>,
}

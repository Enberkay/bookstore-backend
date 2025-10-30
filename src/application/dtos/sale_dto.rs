use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SaleResponse {
    pub id: i32,
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub total_amount: f64,
    pub payment_method: String,
    pub sale_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSaleRequest {
    pub employee_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub payment_method: String,
    pub items: Vec<SaleItemRequest>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SaleItemRequest {
    pub book_isbn: String,
    pub quantity: i32,
    pub price_at_sale: f64,
}

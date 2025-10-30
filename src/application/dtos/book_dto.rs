use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BookResponse {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: Option<String>,
    pub price: f64,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookRequest {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: Option<String>,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookRequest {
    pub title: Option<String>,
    pub author: Option<String>,
    pub synopsis: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
}

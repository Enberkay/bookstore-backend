use serde::{Deserialize, Serialize};
use validator::Validate;
use rust_decimal::prelude::ToPrimitive;
use crate::domain::entities::book::BookEntity;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBookRequest {
    #[validate(length(min = 10, max = 13, message = "ISBN must be 10-13 characters"))]
    pub isbn: String,

    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: String,

    #[validate(length(min = 1, max = 255, message = "Author must be 1-255 characters"))]
    pub author: String,

    #[validate(length(max = 1000, message = "Synopsis too long (max 1000 chars)"))]
    pub synopsis: String,

    #[validate(range(min = 0.01, message = "Price must be at least 0.01"))]
    pub price: f64,

    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBookRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: Option<String>,

    #[validate(length(min = 1, max = 255, message = "Author must be 1-255 characters"))]
    pub author: Option<String>,

    #[validate(length(max = 1000, message = "Synopsis too long (max 1000 chars)"))]
    pub synopsis: Option<String>,

    #[validate(range(min = 0.01, message = "Price must be at least 0.01"))]
    pub price: Option<f64>,

    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BookResponse {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub synopsis: String,
    pub price: f64,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BookEntity> for BookResponse {
    fn from(book: BookEntity) -> Self {
        Self {
            isbn: book.isbn.to_string(),
            title: book.title,
            author: book.author.unwrap_or_default(),
            synopsis: book.synopsis.unwrap_or_default(),
            price: book.price.value().to_f64().unwrap_or(0.0),
            is_active: book.is_active,
            created_at: book.created_at,
            updated_at: book.updated_at,
        }
    }
}
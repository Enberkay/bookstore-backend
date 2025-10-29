use crate::domain::entities::book::BookEntity;
use crate::domain::value_objects::isbn13::Isbn13;
use async_trait::async_trait;

#[async_trait]
pub trait BookRepository: Send + Sync {
    async fn find_by_isbn(&self, isbn: &Isbn13) -> Result<Option<BookEntity>, anyhow::Error>;
    async fn find_all(&self, limit: u32, offset: u32) -> Result<Vec<BookEntity>, anyhow::Error>;
    async fn save(&self, book: &BookEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, isbn: &Isbn13) -> Result<(), anyhow::Error>;
}

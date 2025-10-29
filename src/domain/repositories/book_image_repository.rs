use crate::domain::entities::book_image::BookImageEntity;
use crate::domain::value_objects::isbn13::Isbn13;
use async_trait::async_trait;

#[async_trait]
pub trait BookImageRepository: Send + Sync {
    async fn find_by_book(&self, isbn: &Isbn13) -> Result<Vec<BookImageEntity>, anyhow::Error>;
    async fn add(&self, image: &BookImageEntity) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

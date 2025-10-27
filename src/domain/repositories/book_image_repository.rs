use crate::domain::entities::book_image::BookImageEntity;
use crate::domain::value_objects::isbn13::Isbn13;

pub trait BookImageRepository: Send + Sync {
    fn find_by_book(&self, isbn: &Isbn13) -> Result<Vec<BookImageEntity>, anyhow::Error>;
    fn add(&self, image: &BookImageEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, id: i32) -> Result<(), anyhow::Error>;
}

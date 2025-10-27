use crate::domain::entities::book::BookEntity;
use crate::domain::value_objects::isbn13::Isbn13;

pub trait BookRepository: Send + Sync {
    fn find_by_isbn(&self, isbn: &Isbn13) -> Result<Option<BookEntity>, anyhow::Error>;
    fn find_all(&self, limit: u32, offset: u32) -> Result<Vec<BookEntity>, anyhow::Error>;
    fn save(&self, book: &BookEntity) -> Result<(), anyhow::Error>;
    fn delete(&self, isbn: &Isbn13) -> Result<(), anyhow::Error>;
}

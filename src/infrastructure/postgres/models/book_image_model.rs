use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};

use crate::{
    infrastructure::postgres::schema::book_images,
    domain::{
        entities::book_image::BookImageEntity,
        value_objects::isbn13::Isbn13,
    },
};

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Selectable)]
#[diesel(table_name = book_images)]
#[diesel(primary_key(id))]
pub struct BookImageModel {
    pub id: i32,
    pub book_isbn: String,
    pub image_url: String,
    pub image_type: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

// ==================================
// Mapping between Entity ↔ Model
// ==================================

impl From<BookImageModel> for BookImageEntity {
    fn from(model: BookImageModel) -> Self {
        Self {
            id: model.id,
            book_isbn: Isbn13::new(&model.book_isbn).expect("Invalid ISBN-13"),
            image_url: model.image_url,
            image_type: model.image_type,
            sort_order: model.sort_order,
            created_at: model.created_at,
        }
    }
}

impl From<BookImageEntity> for BookImageModel {
    fn from(entity: BookImageEntity) -> Self {
        Self {
            id: entity.id,
            book_isbn: entity.book_isbn.to_string(),
            image_url: entity.image_url,
            image_type: entity.image_type,
            sort_order: entity.sort_order,
            created_at: entity.created_at,
        }
    }
}

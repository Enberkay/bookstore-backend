use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CategoryEntity {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

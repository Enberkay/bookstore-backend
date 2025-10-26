use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub password: String,
    pub branch_id: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

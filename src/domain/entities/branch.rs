use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct BranchEntity {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
}

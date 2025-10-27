use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RoleEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl RoleEntity {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            name,
            description,
            created_at: Utc::now(),
        }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn update_description(&mut self, desc: Option<String>) {
        self.description = desc;
    }
}

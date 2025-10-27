use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PermissionEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PermissionEntity {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: 0,
            name,
            description,
            created_at: Utc::now(),
        }
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn rename(&mut self, name: String) {
        self.name = name;
    }
}

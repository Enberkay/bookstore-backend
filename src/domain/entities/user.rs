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

impl UserEntity {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        age: i32,
        sex: String,
        phone: String,
        password: String,
        branch_id: Option<i32>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            first_name,
            last_name,
            email,
            age,
            sex,
            phone,
            password,
            branch_id,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    pub fn update_email(&mut self, new_email: String) {
        self.email = new_email;
        self.updated_at = Utc::now();
    }

    pub fn change_password(&mut self, hashed_password: String) {
        self.password = hashed_password;
        self.updated_at = Utc::now();
    }
}

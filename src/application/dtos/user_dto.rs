use serde::{Deserialize, Serialize};
use crate::domain::entities::user::UserEntity;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub password: String,
    pub branch_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub sex: Option<String>,
    pub phone: Option<String>,
    pub branch_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
    pub branch_id: Option<i32>,
    pub is_active: bool,
}

impl From<UserEntity> for UserResponse {
    fn from(user: UserEntity) -> Self {
        Self {
            id: user.id,
            full_name: user.full_name(),
            email: user.email.as_str().to_string(),
            age: user.age,
            sex: user.sex,
            phone: user.phone,
            branch_id: user.branch_id,
            is_active: user.is_active,
        }
    }
}

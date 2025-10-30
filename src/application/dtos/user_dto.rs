use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub roles: Vec<String>,
    pub department_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub password: String,
    pub department_id: Option<i32>,
    pub role_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub email: Option<String>,
    pub department_id: Option<i32>,
}

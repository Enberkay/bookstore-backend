use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub fname: String,
    pub lname: String,
    pub email: String,
    pub password: String,
    pub age: i32,
    pub sex: String,
    pub phone: String,
}


#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
}

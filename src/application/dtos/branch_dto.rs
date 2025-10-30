use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub description: Option<String>,
}

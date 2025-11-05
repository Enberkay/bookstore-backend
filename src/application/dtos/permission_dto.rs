use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;
use crate::domain::entities::permission::PermissionEntity;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 2, max = 50, message = "Permission name must be 2-50 characters"))]
    pub name: String,

    #[validate(length(max = 255, message = "Description too long (max 255 chars)"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePermissionRequest {
    #[validate(length(min = 2, max = 50, message = "Permission name must be 2-50 characters"))]
    pub name: Option<String>,

    #[validate(length(max = 255, message = "Description too long (max 255 chars)"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PermissionEntity> for PermissionResponse {
    fn from(entity: PermissionEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

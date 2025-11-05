use std::sync::Arc;
use crate::application::application_errors::{ApplicationError, ApplicationResult};
use crate::application::dtos::permission_dto::{
    CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest,
};
use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};

pub struct PermissionService {
    repo: Arc<dyn PermissionRepository>,
}

impl PermissionService {
    pub fn new(repo: Arc<dyn PermissionRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_permission(&self, req: CreatePermissionRequest) -> ApplicationResult<PermissionResponse> {
        let mut permission = PermissionEntity::new(req.name, req.description)
            .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        let id = self.repo.save(&permission).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to save permission: {}", e))
        })?;

        // Set the returned ID to the entity
        permission.id = id;

        Ok(PermissionResponse::from(permission))
    }

    pub async fn get_all_permissions(&self) -> ApplicationResult<Vec<PermissionResponse>> {
        let permissions = self.repo.find_all().await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch permissions: {}", e))
        })?;
        Ok(permissions.into_iter().map(PermissionResponse::from).collect())
    }

    pub async fn get_permission_by_id(&self, id: i32) -> ApplicationResult<Option<PermissionResponse>> {
        let permission_opt = self.repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch permission: {}", e))
        })?;
        Ok(permission_opt.map(PermissionResponse::from))
    }

    pub async fn update_permission(&self, id: i32, req: UpdatePermissionRequest) -> ApplicationResult<PermissionResponse> {
        // Validate input first
        if let Some(name) = &req.name {
            let temp_permission = PermissionEntity::new(name.clone(), None)
                .map_err(|e| ApplicationError::bad_request(e.to_string()))?;
            temp_permission.validate().map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        }

        // Use COALESCE update - single query approach
        let updated_permission = self.repo.update(
            id,
            req.name,
            req.description
        ).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to update permission: {}", e))
        })?;

        Ok(PermissionResponse::from(updated_permission))
    }

    pub async fn delete_permission(&self, id: i32) -> ApplicationResult<PermissionResponse> {
        // Get permission before deletion for response
        let permission = match self.repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch permission: {}", e))
        })? {
            Some(p) => p,
            None => return Err(ApplicationError::not_found("Permission not found")),
        };

        // Delete permission
        self.repo.delete(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to delete permission: {}", e))
        })?;

        // Return deleted permission data
        Ok(PermissionResponse::from(permission))
    }
}

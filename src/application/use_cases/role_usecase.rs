use std::sync::Arc;
use anyhow::{Result, anyhow};

use crate::application::dtos::role_dto::{CreateRoleRequest, UpdateRoleRequest, RoleResponse};
use crate::domain::{
    entities::role::RoleEntity,
    repositories::role_repository::RoleRepository,
};

/// RoleUseCase — encapsulates application-level business logic for managing roles
pub struct RoleUseCase {
    role_repo: Arc<dyn RoleRepository>,
}

impl RoleUseCase {
    pub fn new(role_repo: Arc<dyn RoleRepository>) -> Self {
        Self { role_repo }
    }

    /// Create a new role
    pub async fn create_role(&self, req: CreateRoleRequest) -> Result<RoleResponse> {
        // สร้าง entity พร้อม validate
        let mut role = RoleEntity::new(req.name, req.description)
            .map_err(|e| anyhow!(e.to_string()))?;

        // Save role
        let role_id = self
            .role_repo
            .save(&role)
            .await
            .map_err(|e| anyhow!(format!("Failed to save role: {}", e)))?;

        role.id = role_id;

        Ok(RoleResponse::from(role))
    }

    /// Get role by ID
    pub async fn get_role_by_id(&self, id: i32) -> Result<Option<RoleResponse>> {
        let role_opt = self.role_repo.find_by_id(id).await.map_err(|e| {
            anyhow!(format!("Database error while fetching role: {}", e))
        })?;

        Ok(role_opt.map(RoleResponse::from))
    }

    /// Get all roles
    pub async fn get_all_roles(&self) -> Result<Vec<RoleResponse>> {
        let roles = self.role_repo.find_all().await.map_err(|e| {
            anyhow!(format!("Failed to fetch all roles: {}", e))
        })?;

        Ok(roles.into_iter().map(RoleResponse::from).collect())
    }

    /// Update role
    pub async fn update_role(
        &self,
        id: i32,
        req: UpdateRoleRequest,
    ) -> Result<RoleResponse> {
        // ตรวจว่า role มีอยู่จริง
        let _ = match self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Database error: {}", e)))?
        {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        // Validate name ถ้ามีการส่งมา
        if let Some(ref name) = req.name {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err(anyhow!("Role name cannot be empty"));
            }
            if trimmed.len() > 100 {
                return Err(anyhow!("Role name too long (max 100 chars)"));
            }
        }

        // Update role
        let updated_role = self
            .role_repo
            .update(id, req.name, req.description)
            .await
            .map_err(|e| anyhow!(format!("Failed to update role: {}", e)))?;

        Ok(RoleResponse::from(updated_role))
    }

    /// Delete role
    pub async fn delete_role(&self, id: i32) -> Result<RoleResponse> {
        let role = match self
            .role_repo
            .find_by_id(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to fetch role: {}", e)))?
        {
            Some(r) => r,
            None => return Err(anyhow!("Role not found")),
        };

        self.role_repo
            .delete(id)
            .await
            .map_err(|e| anyhow!(format!("Failed to delete role: {}", e)))?;

        Ok(RoleResponse::from(role))
    }
}

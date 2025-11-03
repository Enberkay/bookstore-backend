use std::sync::Arc;
use crate::application::application_errors::{ApplicationError, ApplicationResult};
use crate::application::dtos::user_dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::domain::{
    entities::user::UserEntity,
    repositories::{role_repository::RoleRepository, user_repository::UserRepository},
};

/// UserService — encapsulates application-level business logic
/// for managing users (create, read, update, delete, and role assignment)
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    role_repo: Arc<dyn RoleRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>, role_repo: Arc<dyn RoleRepository>) -> Self {
        Self { user_repo, role_repo }
    }

    /// Create a new user (password hashing handled externally)
    pub async fn create_user(&self, req: CreateUserRequest) -> ApplicationResult<UserResponse> {
        // ตรวจว่ามีอีเมลซ้ำไหม
        if let Some(_) = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while checking email: {}", e))
        })? {
            return Err(ApplicationError::conflict("Email already exists"));
        }

        let user = UserEntity::new(
            req.first_name,
            req.last_name,
            req.email,
            req.age,
            req.sex,
            req.phone,
            req.password,
            req.branch_id,
        )
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        self.user_repo.save(&user).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to save user: {}", e))
        })?;

        Ok(UserResponse::from(user))
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: i32) -> ApplicationResult<Option<UserResponse>> {
        let user_opt = self.user_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while fetching user: {}", e))
        })?;
        Ok(user_opt.map(UserResponse::from))
    }

    /// Get all users
    pub async fn get_all_users(&self) -> ApplicationResult<Vec<UserResponse>> {
        let users = self.user_repo.find_all().await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch all users: {}", e))
        })?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Update user profile (except password)
    pub async fn update_user(&self, id: i32, req: UpdateUserRequest) -> ApplicationResult<()> {
        let mut user = match self.user_repo.find_by_id(id).await.map_err(|e| {
            ApplicationError::internal(format!("Database error: {}", e))
        })? {
            Some(u) => u,
            None => return Err(ApplicationError::not_found("User not found")),
        };

        if let Some(fname) = req.first_name {
            user.first_name = fname;
        }
        if let Some(lname) = req.last_name {
            user.last_name = lname;
        }
        if let Some(email) = req.email {
            user.update_email(email)
                .map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        }
        if let Some(age) = req.age {
            user.age = age;
        }
        if let Some(sex) = req.sex {
            user.sex = sex.to_uppercase();
        }
        if let Some(phone) = req.phone {
            user.update_phone(phone)
                .map_err(|e| ApplicationError::bad_request(e.to_string()))?;
        }
        if let Some(branch_id) = req.branch_id {
            user.branch_id = Some(branch_id);
        }

        self.user_repo.update(&user).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to update user: {}", e))
        })?;

        Ok(())
    }

    /// Delete user
    pub async fn delete_user(&self, id: i32) -> ApplicationResult<()> {
        self.user_repo.delete(id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to delete user: {}", e))
        })
    }

    // =====================================================
    // RBAC FEATURES
    // =====================================================

    /// Assign roles to a user
    pub async fn assign_roles(&self, user_id: i32, role_ids: Vec<i32>) -> ApplicationResult<()> {
        let user_opt = self.user_repo.find_by_id(user_id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user: {}", e))
        })?;

        if user_opt.is_none() {
            return Err(ApplicationError::not_found("User not found"));
        }

        let roles = self.role_repo.find_by_ids(&role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch roles: {}", e))
        })?;

        if roles.len() != role_ids.len() {
            return Err(ApplicationError::bad_request("Some roles not found".to_string()));
        }

        self.user_repo.assign_roles(user_id, &role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to assign roles: {}", e))
        })
    }

    /// Remove roles from a user
    pub async fn remove_roles(&self, user_id: i32, role_ids: Vec<i32>) -> ApplicationResult<()> {
        self.user_repo.remove_roles(user_id, &role_ids).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to remove roles: {}", e))
        })
    }

    /// Get all roles assigned to a user
    pub async fn get_user_roles(&self, user_id: i32) -> ApplicationResult<Vec<String>> {
        let roles = self.user_repo.find_roles(user_id).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to fetch user roles: {}", e))
        })?;
        Ok(roles.into_iter().map(|r| r.name).collect())
    }
}

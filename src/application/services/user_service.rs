use std::sync::Arc;
use anyhow::Result;

use crate::application::dtos::user_dto::{
    CreateUserRequest,
    UpdateUserRequest,
    UserResponse,
};
use crate::domain::{
    entities::user::UserEntity,
    repositories::user_repository::UserRepository,
};

/// UserService — encapsulates application-level business logic
/// for managing users (create, read, update, delete).
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Create a new user (without handling password hashing — AuthService handles that)
    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse> {
        let user = UserEntity::new(
            req.first_name,
            req.last_name,
            req.email,
            req.age,
            req.sex,
            req.phone,
            req.password,
            req.branch_id,
        )?;

        self.user_repo.save(&user).await?;

        Ok(UserResponse::from(user))
    }

    /// Get a user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<Option<UserResponse>> {
        let user_opt = self.user_repo.find_by_id(id).await?;
        Ok(user_opt.map(UserResponse::from))
    }

    /// Get all users
    pub async fn get_all_users(&self) -> Result<Vec<UserResponse>> {
        let users = self.user_repo.find_all().await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    /// Update an existing user (except password)
    pub async fn update_user(&self, id: i32, req: UpdateUserRequest) -> Result<()> {
        let mut user = match self.user_repo.find_by_id(id).await? {
            Some(u) => u,
            None => anyhow::bail!("User not found"),
        };

        if let Some(fname) = req.first_name {
            user.first_name = fname;
        }
        if let Some(lname) = req.last_name {
            user.last_name = lname;
        }
        if let Some(email) = req.email {
            user.update_email(email)?;
        }
        if let Some(age) = req.age {
            user.age = age;
        }
        if let Some(sex) = req.sex {
            user.sex = sex.to_uppercase();
        }
        if let Some(phone) = req.phone {
            user.update_phone(phone)?;
        }
        if let Some(branch_id) = req.branch_id {
            user.branch_id = Some(branch_id);
        }

        self.user_repo.save(&user).await
    }

    /// Delete a user
    pub async fn delete_user(&self, id: i32) -> Result<()> {
        self.user_repo.delete(id).await
    }
}

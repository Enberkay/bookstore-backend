use std::sync::Arc;
use crate::application::{
    dtos::auth_dto::{LoginRequest, LoginResponse, RefreshResponse, RegisterRequest, RegisterResponse, UserInfo},
};
use anyhow::{Result, anyhow, Context};
use crate::{
    domain::repositories::{
        user_repository::UserRepository,
    },
    infrastructure::{
        argon2::PasswordService,
        jwt::JwtService,
    },
    domain::entities::user::UserEntity,
};

/// AuthUseCase จัดการ Authentication flow ทั้งหมด (AT/RT Stateless)
pub struct AuthUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_repo: Arc<dyn PasswordService>,
    jwt_repo: Arc<dyn JwtService>,
}

impl AuthUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_repo: Arc<dyn PasswordService>,
        jwt_repo: Arc<dyn JwtService>,
    ) -> Self {
        Self {
            user_repo,
            password_repo,
            jwt_repo,
        }
    }

    /// สมัครสมาชิกใหม่
    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse> {
        // DB Call (Async)
        if self.user_repo.find_by_email(&req.email).await
            .context("Database error while checking email")?
            .is_some() 
        {
            return Err(anyhow!("Email already exists"));
        }

        // Hashing (Async ถ้าใช้ spawn_blocking ใน implementation, หรือ Sync ก็ได้แล้วแต่ implement)
        // สมมติ PasswordService ยังเป็น Async ตามเดิม
        let hashed_password = self.password_repo.hash_password(&req.password).await
            .context("Failed to hash password")?;

        let user = UserEntity::new(
            req.fname,
            req.lname,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
        ).map_err(|e| anyhow!("{}", e))?;

        // DB Call (Async)
        let user_id = self.user_repo.save(&user).await
            .context("Failed to save user")?;

        Ok(RegisterResponse {
            id: user_id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(), 
            lname: user.last_name.as_str().to_string(),
        })
    }

    /// เข้าสู่ระบบ
    pub async fn login(&self, req: LoginRequest) -> Result<(LoginResponse, String)> {
        // 1. DB Call (Async)
        let user = self.user_repo.find_by_email(&req.email).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        // 2. Password Check (Async)
        let valid = self.password_repo.verify_password(&req.password, user.password.as_str()).await
            .context("Failed to verify password")?;

        if !valid {
            return Err(anyhow!("Invalid credentials"));
        }

        // 3. DB Call (Async)
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string()) 
            .collect();

        // 4. JWT Generation (Sync - ไม่มี .await แล้ว!)
        // นี่คือจุดที่แก้ครับ
        let access_token = self.jwt_repo
            .generate_access_token(user.id, &role_names) // <-- No await
            .context("Failed to create access token")?;

        // 5. JWT Generation (Sync - ไม่มี .await แล้ว!)
        let refresh_token = self.jwt_repo
            .generate_refresh_token(user.id) // <-- No await
            .context("Failed to create refresh token")?;

        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        };

        Ok((LoginResponse {
            user: user_info,
            access_token,
        }, refresh_token))
    }

    /// Refresh token flow
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<RefreshResponse> {
        
        // 1. Validate JWT (Sync - ไม่มี .await แล้ว!)
        // คืนค่า user_id (i32) ตรงๆ
        let user_id = self.jwt_repo.validate_refresh_token(refresh_token) // <-- No await
            .context("Invalid or expired refresh token")?;

        // 2. DB Check (Async)
        let user = self.user_repo.find_by_id(user_id).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("User not found or account deactivated"))?;

        // 3. DB Check Roles (Async)
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string())
            .collect();

        // 4. Issue New AT (Sync - ไม่มี .await แล้ว!)
        let new_access_token = self.jwt_repo
            .generate_access_token(user.id, &role_names) // <-- No await
            .context("Failed to create new access token")?;

        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        };

        Ok(RefreshResponse {
            user: user_info,
            access_token: new_access_token,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<UserInfo> {
        // 1. Validate JWT (Sync - ไม่มี .await แล้ว!)
        let claims = self.jwt_repo.validate_access_token(token) // <-- No await
            .context("Invalid or expired access token")?;

        // 2. Parse ID
        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| anyhow!("Invalid user ID format in token"))?;

        // 3. DB Call (Async)
        let user = self.user_repo.find_by_id(user_id).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("User not found"))?;

        // 4. DB Call (Async)
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string())
            .collect();

        Ok(UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        })
    }
}
use std::sync::Arc;
use anyhow::Result;

use crate::application::dtos::auth_dto::{
    LoginRequest,
    LoginResponse,
    RegisterRequest,
    RegisterResponse,
};
use crate::domain::repositories::{
    user_repository::UserRepository,
    password_repository::PasswordRepository,
    token_repository::{JwtRepository, TokenRepository},
};
use crate::domain::entities::refresh_token::NewRefreshToken;

/// AuthService จัดการ Authentication flow ทั้งหมด
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    password_repo: Arc<dyn PasswordRepository>,
    jwt_repo: Arc<dyn JwtRepository>,
    token_repo: Arc<dyn TokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_repo: Arc<dyn PasswordRepository>,
        jwt_repo: Arc<dyn JwtRepository>,
        token_repo: Arc<dyn TokenRepository>,
    ) -> Self {
        Self {
            user_repo,
            password_repo,
            jwt_repo,
            token_repo,
        }
    }

    /// สมัครสมาชิกใหม่
    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse> {
        // hash password
        let hashed_password = self.password_repo.hash(&req.password).await?;

        //สร้าง user entity ให้ตรงกับ signature
        let user = crate::domain::entities::user::UserEntity::new(
            req.fname,
            req.lname,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
            None,
        )?;

        // save user
        self.user_repo.save(&user).await?;

        Ok(RegisterResponse {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
        })
    }


    /// เข้าสู่ระบบ
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse> {
        let user_opt = self.user_repo.find_by_email(&req.email).await?;
        let user = match user_opt {
            Some(u) => u,
            None => anyhow::bail!("Invalid credentials"),
        };

        // verify password
        let valid = self.password_repo.verify(&req.password, &user.password).await?;
        if !valid {
            anyhow::bail!("Invalid credentials");
        }

        // UserEntity ไม่มี roles / permissions — ใช้ค่า default แทน
        let roles: Vec<String> = Vec::new();
        let permissions: Vec<String> = Vec::new();

        // create access token
        let access_token = self
            .jwt_repo
            .create_access_token(user.id, &roles, &permissions)
            .await?;

        // create refresh token (plain)
        let refresh_token = self.jwt_repo.create_refresh_token(user.id, 7).await?;
        let refresh_token_hash = self.jwt_repo.hash_refresh_token(&refresh_token).await?;

        // store refresh token in redis
        let token_data = NewRefreshToken {
            user_id: user.id,
            token_hash: refresh_token_hash,
            expires_at: chrono::Utc::now() + chrono::Duration::days(7),
        };

        self.token_repo.store_refresh_token(token_data).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
        })
    }

    /// Validate token and return user id
    pub async fn validate_token(&self, token: &str) -> Result<i32> {
        let user_id = self.jwt_repo.validate_access_token(token).await?;
        Ok(user_id)
    }

    /// Logout (revoke refresh token)
    pub async fn logout(&self, refresh_token_hash: &str) -> Result<()> {
        self.token_repo.revoke_token(refresh_token_hash).await
    }
}

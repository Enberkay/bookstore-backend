use anyhow::{bail, Result};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
    pub users_secret: UsersSecret,
    pub jwt: JwtConfig,
    pub environment: Environment,
}

impl AppConfig {
    pub fn validate(&self) -> Result<()> {
        self.server.validate()?;
        self.database.validate()?;
        self.jwt.validate()?;
        self.users_secret.validate()?;

        // cross-field validation
        if self.jwt.access_token_expiry_minutes > 60 * 24 {
            bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must not exceed 24 hours");
        }

        if matches!(self.environment, Environment::Production | Environment::Staging) {
            if self.server.cors_allowed_origins.contains(&"*".to_string()) {
                bail!("CORS cannot allow wildcard (*) in production or staging");
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout_seconds: u32,
    pub cors_allowed_origins: Vec<String>,
}

impl Server {
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            bail!("SERVER_PORT must be greater than 0");
        }
        if self.body_limit == 0 {
            bail!("SERVER_BODY_LIMIT must be greater than 0");
        }
        if self.timeout_seconds == 0 {
            bail!("SERVER_TIMEOUT must be greater than 0");
        }

        if self.cors_allowed_origins.iter().any(|o| o != "*" && !o.starts_with("http")) {
            bail!("CORS origins must start with http:// or https:// or be '*'");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

impl Database {
    pub fn validate(&self) -> Result<()> {
        if self.url.trim().is_empty() {
            bail!("DATABASE_URL cannot be empty");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UsersSecret {
    pub secret: String,
    pub refresh_secret: String,
}

impl UsersSecret {
    pub fn validate(&self) -> Result<()> {
        if self.secret.len() < 32 {
            bail!("JWT_USERS_SECRET must be at least 32 characters");
        }
        if self.refresh_secret.len() < 32 {
            bail!("JWT_USERS_REFRESH_SECRET must be at least 32 characters");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_expiry_minutes: u64,
    pub refresh_token_expiry_days: u64,
}

impl JwtConfig {
    pub fn validate(&self) -> Result<()> {
        if self.access_token_expiry_minutes == 0 {
            bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must be greater than 0");
        }
        if self.refresh_token_expiry_days == 0 {
            bail!("JWT_REFRESH_TOKEN_EXPIRY_DAYS must be greater than 0");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Ok(Self::Development),
            "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Production),
            _ => bail!("Invalid ENVIRONMENT value: {}", s),
        }
    }
}

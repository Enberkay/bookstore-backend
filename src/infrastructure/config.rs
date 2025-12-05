use anyhow::{bail, Context, Result};
use std::{env, str::FromStr};

// Configuration Models
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
    pub jwt: JwtConfig,
    pub environment: Environment,
}

impl AppConfig {
    pub fn validate(&self) -> Result<()> {
        self.server.validate()?;
        self.database.validate()?;
        self.jwt.validate()?;

        // cross-field validation
        if self.jwt.access_token_expiry_minutes > 60 * 24 {
            bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must not exceed 24 hours");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            bail!("SERVER_PORT must be greater than 0");
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
pub struct JwtConfig {
    pub access_token_expiry_minutes: u64,
    pub refresh_token_expiry_days: u64,
    pub secret: String,
    pub refresh_secret: String,
}

impl JwtConfig {
    pub fn validate(&self) -> Result<()> {
        if self.access_token_expiry_minutes == 0 {
            bail!("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must be greater than 0");
        }
        if self.refresh_token_expiry_days == 0 {
            bail!("JWT_REFRESH_TOKEN_EXPIRY_DAYS must be greater than 0");
        }
        if self.secret.len() < 32 {
            bail!("JWT_SECRET must be at least 32 characters");
        }
        if self.refresh_secret.len() < 32 {
            bail!("JWT_REFRESH_SECRET must be at least 32 characters");
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

// Configuration Loader
pub fn load() -> Result<AppConfig> {
    let server = Server {
        port: env::var("SERVER_PORT")?.parse().context("SERVER_PORT must be a number")?,
    };

    let database = Database {
        url: env::var("DATABASE_URL").context("DATABASE_URL is required")?,
    };

    let jwt = JwtConfig {
        access_token_expiry_minutes: env::var("JWT_ACCESS_TOKEN_EXPIRY_MINUTES")?
            .parse()
            .context("JWT_ACCESS_TOKEN_EXPIRY_MINUTES must be a number")?,
        refresh_token_expiry_days: env::var("JWT_REFRESH_TOKEN_EXPIRY_DAYS")?
            .parse()
            .context("JWT_REFRESH_TOKEN_EXPIRY_DAYS must be a number")?,
        secret: env::var("JWT_SECRET").context("JWT_SECRET is required")?,
        refresh_secret: env::var("JWT_REFRESH_SECRET").context("JWT_REFRESH_SECRET is required")?,
    };

    let environment = env::var("ENVIRONMENT")?
        .parse::<Environment>()?;

    let config = AppConfig {
        server,
        database,
        jwt,
        environment,
    };

    config.validate()?;
    Ok(config)
}

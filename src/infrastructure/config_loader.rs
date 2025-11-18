use crate::infrastructure::config_model::*;
use anyhow::{Context, Result};
use std::env;

pub fn load() -> Result<AppConfig> {
    let server = Server {
        port: env::var("SERVER_PORT")?.parse().context("SERVER_PORT must be a number")?,
        body_limit: env::var("SERVER_BODY_LIMIT")?.parse().context("SERVER_BODY_LIMIT must be a number")?,
        timeout_seconds: env::var("SERVER_TIMEOUT")?.parse().context("SERVER_TIMEOUT must be a number")?,
        cors_allowed_origins: env::var("SERVER_CORS_ALLOWED_ORIGINS")?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
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
    };

    let environment = env::var("ENVIRONMENT")?
        .parse::<Environment>()?;

    let users_secret = UsersSecret {
        secret: env::var("JWT_USERS_SECRET").context("JWT_USERS_SECRET is required")?,
        refresh_secret: env::var("JWT_USERS_REFRESH_SECRET").context("JWT_USERS_REFRESH_SECRET is required")?,
    };

    let config = AppConfig {
        server,
        database,
        jwt,
        environment,
        users_secret,
    };

    config.validate()?;
    Ok(config)
}

pub struct AppConfig {
    pub server: Server,
    pub database: Database,
    pub redis: Reids,
    pub users_secret: UsersSecret,
    pub jwt: JwtConfig,
    pub environment: Environment,
    pub security: SecurityConfig,
    pub production: ProductionConfig,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout_seconds: u32,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct Reids {
    pub url: String,
    pub max_connections: u32,
    pub refresh_token_expiry_days: u64,
}

#[derive(Debug, Clone)]
pub struct UsersSecret {
    pub secret: String,
    pub refresh_secret: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_token_expiry_minutes: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub rate_limit_requests_per_minute: u32,
    pub argon2_memory_cost: u32,
    pub argon2_time_cost: u32,
    pub argon2_parallelism: u32,
}

#[derive(Debug, Clone)]
pub struct ProductionConfig {
    pub https_redirect: bool,
    pub trust_proxy: bool,
}

use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tower::{Layer, Service};
use tokio::sync::RwLock;

/// Rate Limiter Configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    pub max_requests: u32,
    pub window_duration: Duration,
    pub cleanup_interval: Duration,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests: 100, // 100 requests per window
            window_duration: Duration::from_secs(60), // 1 minute window
            cleanup_interval: Duration::from_secs(300), // 5 minutes cleanup
        }
    }
}

impl RateLimiterConfig {
    /// Create from security configuration
    pub fn from_security_config(rate_limit_requests_per_minute: u32) -> Self {
        Self {
            max_requests: rate_limit_requests_per_minute,
            window_duration: Duration::from_secs(60), // 1 minute window
            cleanup_interval: Duration::from_secs(300), // 5 minutes cleanup
        }
    }
}

/// Rate Limiter for Authentication Endpoints
#[derive(Clone)]
pub struct RateLimiterLayer {
    config: RateLimiterConfig,
    store: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiterLayer {
    pub fn new(config: RateLimiterConfig) -> Self {
        let store = Arc::new(RwLock::new(HashMap::new()));

        // Start cleanup task
        let cleanup_store = store.clone();
        let cleanup_interval = config.cleanup_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                cleanup_expired_entries(&cleanup_store).await;
            }
        });

        Self { config, store }
    }

    pub fn for_auth_endpoints() -> Self {
        Self::new(RateLimiterConfig {
            max_requests: 5, // Default: 5 login attempts per minute
            window_duration: Duration::from_secs(60), // 1 minute window
            cleanup_interval: Duration::from_secs(300), // 5 minutes cleanup
        })
    }

    pub fn for_auth_endpoints_with_config(config: RateLimiterConfig) -> Self {
        Self::new(config)
    }

    pub fn for_general_endpoints() -> Self {
        Self::new(RateLimiterConfig::default())
    }
}

#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

async fn cleanup_expired_entries(store: &Arc<RwLock<HashMap<String, RateLimitEntry>>>) {
    let mut store = store.write().await;
    let now = Instant::now();

    store.retain(|_, entry| {
        now.duration_since(entry.window_start) < Duration::from_secs(300) // 5 minutes
    });
}

impl<S> Layer<S> for RateLimiterLayer {
    type Service = RateLimiter<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimiter {
            inner,
            config: self.config.clone(),
            store: self.store.clone(),
        }
    }
}

/// Rate Limiter Service
#[derive(Clone)]
pub struct RateLimiter<S> {
    inner: S,
    config: RateLimiterConfig,
    store: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl<S> Service<Request> for RateLimiter<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let config = self.config.clone();
        let store = self.store.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Get client IP
            let client_ip = req
                .headers()
                .get("x-forwarded-for")
                .or_else(|| req.headers().get("x-real-ip"))
                .and_then(|h| h.to_str().ok())
                .unwrap_or("unknown")
                .to_string();

            // Check rate limit
            if let Err(response) = check_rate_limit(&store, &client_ip, &config).await {
                return Ok(response);
            }

            // Continue with request
            inner.call(req).await
        })
    }
}

async fn check_rate_limit(
    store: &Arc<RwLock<HashMap<String, RateLimitEntry>>>,
    client_ip: &str,
    config: &RateLimiterConfig,
) -> Result<(), Response> {
    let now = Instant::now();
    let mut store = store.write().await;

    let entry = store.get_mut(client_ip);

    match entry {
        Some(entry) => {
            // Check if window has expired
            if now.duration_since(entry.window_start) >= config.window_duration {
                // Reset window
                entry.count = 1;
                entry.window_start = now;
            } else {
                // Check if limit exceeded
                if entry.count >= config.max_requests {
                    let error_response = serde_json::json!({
                        "success": false,
                        "message": "Rate limit exceeded",
                        "error": format!("Too many requests. Limit: {} per {} seconds",
                                       config.max_requests,
                                       config.window_duration.as_secs()),
                        "retry_after": config.window_duration.as_secs()
                    });

                    return Err((StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response());
                }

                // Increment counter
                entry.count += 1;
            }
        }
        None => {
            // First request from this IP
            store.insert(client_ip.to_string(), RateLimitEntry {
                count: 1,
                window_start: now,
            });
        }
    }

    Ok(())
}

/// Account Lockout Configuration
#[derive(Debug, Clone)]
pub struct AccountLockoutConfig {
    pub max_failed_attempts: u32,
    pub lockout_duration: Duration,
    pub cleanup_interval: Duration,
}

impl Default for AccountLockoutConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5, // Default: 5 failed attempts before lockout
            lockout_duration: Duration::from_secs(900), // Default: 15 minutes lockout
            cleanup_interval: Duration::from_secs(300), // Default: 5 minutes cleanup
        }
    }
}

/// Account Lockout for Failed Login Attempts
#[derive(Clone)]
pub struct AccountLockoutLayer {
    config: AccountLockoutConfig,
    store: Arc<RwLock<HashMap<String, LockoutEntry>>>,
}

#[derive(Debug, Clone)]
pub struct LockoutEntry {
    failed_attempts: u32,
    locked_until: Option<Instant>,
    last_attempt: Instant,
}

impl AccountLockoutLayer {
    pub fn new(config: AccountLockoutConfig) -> Self {
        let store = Arc::new(RwLock::new(HashMap::new()));

        // Start cleanup task
        let cleanup_store = store.clone();
        let cleanup_interval = config.cleanup_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                cleanup_expired_lockouts(&cleanup_store).await;
            }
        });

        Self { config, store }
    }

    pub fn create_default() -> Self {
        Self::new(AccountLockoutConfig::default())
    }
}

impl Default for AccountLockoutLayer {
    fn default() -> Self {
        Self::new(AccountLockoutConfig::default())
    }
}

async fn cleanup_expired_lockouts(store: &Arc<RwLock<HashMap<String, LockoutEntry>>>) {
    let mut store = store.write().await;
    let now = Instant::now();

    store.retain(|_, entry| {
        if let Some(locked_until) = entry.locked_until && now >= locked_until {
            // Lockout expired, reset attempts
            entry.failed_attempts = 0;
            entry.locked_until = None;
        }

        // Keep entry if it's recent (within 1 hour)
        now.duration_since(entry.last_attempt) < Duration::from_secs(3600)
    });
}

impl<S> Layer<S> for AccountLockoutLayer {
    type Service = AccountLockout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AccountLockout {
            inner,
            config: self.config.clone(),
            store: self.store.clone(),
        }
    }
}

/// Account Lockout Service
#[derive(Clone)]
pub struct AccountLockout<S> {
    inner: S,
    config: AccountLockoutConfig,
    store: Arc<RwLock<HashMap<String, LockoutEntry>>>,
}

impl<S> Service<Request> for AccountLockout<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let config = self.config.clone();
        let store = self.store.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Only apply to login endpoint
            if req.uri().path().ends_with("/login") {
                // Extract email from request body (simplified)
                // In real implementation, you'd parse the JSON body
                let client_ip = req
                    .headers()
                    .get("x-forwarded-for")
                    .or_else(|| req.headers().get("x-real-ip"))
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("unknown")
                    .to_string();

                // Check if account is locked
                if let Err(response) = check_account_lockout(&store, &client_ip, &config).await {
                    return Ok(response);
                }
            }

            // Continue with request
            inner.call(req).await
        })
    }
}

async fn check_account_lockout(
    store: &Arc<RwLock<HashMap<String, LockoutEntry>>>,
    client_ip: &str,
    _config: &AccountLockoutConfig,
) -> Result<(), Response> {
    let now = Instant::now();
    let mut store = store.write().await;

    let entry = store.get_mut(client_ip);

    match entry {
        Some(entry) => {
            // Check if account is currently locked
            if let Some(locked_until) = entry.locked_until {
                if now < locked_until {
                    let remaining = locked_until.duration_since(now);
                    let error_response = serde_json::json!({
                        "success": false,
                        "message": "Account temporarily locked",
                        "error": format!("Too many failed login attempts. Try again in {} seconds",
                                       remaining.as_secs()),
                        "retry_after": remaining.as_secs()
                    });

                    return Err((StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response());
                } else {
                    // Lockout expired, reset
                    entry.failed_attempts = 0;
                    entry.locked_until = None;
                }
            }
        }
        None => {
            // First attempt from this IP
            store.insert(client_ip.to_string(), LockoutEntry {
                failed_attempts: 0,
                locked_until: None,
                last_attempt: now,
            });
        }
    }

    Ok(())
}

/// Record failed login attempt
pub async fn record_failed_attempt(
    store: &Arc<RwLock<HashMap<String, LockoutEntry>>>,
    client_ip: &str,
    config: &AccountLockoutConfig,
) {
    let now = Instant::now();
    let mut store = store.write().await;

    let entry = store.entry(client_ip.to_string()).or_insert_with(|| LockoutEntry {
        failed_attempts: 0,
        locked_until: None,
        last_attempt: now,
    });

    entry.failed_attempts += 1;
    entry.last_attempt = now;

    if entry.failed_attempts >= config.max_failed_attempts {
        entry.locked_until = Some(now + config.lockout_duration);
    }
}

/// Record successful login (reset failed attempts)
pub async fn record_successful_login(
    store: &Arc<RwLock<HashMap<String, LockoutEntry>>>,
    client_ip: &str,
) {
    let mut store = store.write().await;
    if let Some(entry) = store.get_mut(client_ip) {
        entry.failed_attempts = 0;
        entry.locked_until = None;
    }
}

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{extract::Request, http::HeaderValue, response::Response};
use tower::{Layer, Service};

/// Security Headers Configuration
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    pub content_security_policy: Option<String>,
    pub x_frame_options: Option<String>,
    pub x_content_type_options: Option<String>,
    pub x_xss_protection: Option<String>,
    pub referrer_policy: Option<String>,
    pub strict_transport_security: Option<String>,
    pub permissions_policy: Option<String>,
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            content_security_policy: Some("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'none';".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            strict_transport_security: Some("max-age=31536000; includeSubDomains; preload".to_string()),
            permissions_policy: Some("camera=(), microphone=(), geolocation=(), payment=(), usb=()".to_string()),
        }
    }
}

/// Security Headers Layer
#[derive(Clone)]
pub struct SecurityHeadersLayer {
    config: SecurityHeadersConfig,
}

impl SecurityHeadersLayer {
    pub fn new(config: SecurityHeadersConfig) -> Self {
        Self { config }
    }

    pub fn create_default() -> Self {
        Self::new(SecurityHeadersConfig::default())
    }
}

impl Default for SecurityHeadersLayer {
    fn default() -> Self {
        Self::new(SecurityHeadersConfig::default())
    }
}

impl SecurityHeadersLayer {
    pub fn minimal() -> Self {
        Self::new(SecurityHeadersConfig {
            content_security_policy: None,
            x_frame_options: Some("SAMEORIGIN".to_string()),
            x_content_type_options: Some("nosniff".to_string()),
            x_xss_protection: Some("1; mode=block".to_string()),
            referrer_policy: Some("strict-origin-when-cross-origin".to_string()),
            strict_transport_security: None,
            permissions_policy: None,
        })
    }
}

impl<S> Layer<S> for SecurityHeadersLayer {
    type Service = SecurityHeaders<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecurityHeaders {
            inner,
            config: self.config.clone(),
        }
    }
}

/// Security Headers Service
#[derive(Clone)]
pub struct SecurityHeaders<S> {
    inner: S,
    config: SecurityHeadersConfig,
}

impl<S> Service<Request> for SecurityHeaders<S>
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
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let mut response = inner.call(req).await?;

            // Add security headers
            add_security_headers(&mut response, &config);

            Ok(response)
        })
    }
}

fn add_security_headers(response: &mut Response, config: &SecurityHeadersConfig) {
    let headers = response.headers_mut();

    // Content Security Policy
    if let Some(csp) = &config.content_security_policy
        && let Ok(header_value) = HeaderValue::from_str(csp)
    {
        let _ = headers.insert("Content-Security-Policy", header_value);
    }

    // X-Frame-Options
    if let Some(xfo) = &config.x_frame_options
        && let Ok(header_value) = HeaderValue::from_str(xfo)
    {
        let _ = headers.insert("X-Frame-Options", header_value);
    }

    // X-Content-Type-Options
    if let Some(xcto) = &config.x_content_type_options
        && let Ok(header_value) = HeaderValue::from_str(xcto)
    {
        let _ = headers.insert("X-Content-Type-Options", header_value);
    }

    // X-XSS-Protection
    if let Some(xss) = &config.x_xss_protection
        && let Ok(header_value) = HeaderValue::from_str(xss)
    {
        let _ = headers.insert("X-XSS-Protection", header_value);
    }

    // Referrer-Policy
    if let Some(rp) = &config.referrer_policy
        && let Ok(header_value) = HeaderValue::from_str(rp)
    {
        let _ = headers.insert("Referrer-Policy", header_value);
    }

    // Strict-Transport-Security (only for HTTPS)
    if let Some(hsts) = &config.strict_transport_security
        && let Ok(header_value) = HeaderValue::from_str(hsts)
    {
        let _ = headers.insert("Strict-Transport-Security", header_value);
    }

    // Permissions-Policy
    if let Some(pp) = &config.permissions_policy
        && let Ok(header_value) = HeaderValue::from_str(pp)
    {
        let _ = headers.insert("Permissions-Policy", header_value);
    }

    // Additional security headers
    let _ = headers.insert(
        "X-Permitted-Cross-Domain-Policies",
        HeaderValue::from_static("none"),
    );
    let _ = headers.insert("X-Download-Options", HeaderValue::from_static("noopen"));
    let _ = headers.insert("X-DNS-Prefetch-Control", HeaderValue::from_static("off"));
}

/// CORS Configuration for Security
#[derive(Debug, Clone)]
pub struct SecureCorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub exposed_headers: Vec<String>,
    pub max_age: Option<u64>,
    pub allow_credentials: bool,
}

impl Default for SecureCorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
            exposed_headers: vec![],
            max_age: Some(86400), // 24 hours
            allow_credentials: false,
        }
    }
}

impl SecureCorsConfig {
    pub fn strict() -> Self {
        Self {
            allowed_origins: vec!["https://yourdomain.com".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            exposed_headers: vec![],
            max_age: Some(3600), // 1 hour
            allow_credentials: true,
        }
    }

    pub fn development() -> Self {
        Self {
            allowed_origins: vec![
                "http://localhost:3000".to_string(),
                "http://localhost:8080".to_string(),
            ], // Common dev ports
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
                "X-Forwarded-For".to_string(),
                "X-Real-IP".to_string(),
            ],
            exposed_headers: vec![],
            max_age: Some(3600),
            allow_credentials: true,
        }
    }
}

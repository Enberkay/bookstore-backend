use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    Json,
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower::{Layer, Service};

use crate::infrastructure::jwt_authentication::validate_access_token_claims;

#[derive(Clone)]
pub struct JwtAuthLayer {
    jwt_secret: String,
}

impl JwtAuthLayer {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S> Layer<S> for JwtAuthLayer {
    type Service = JwtAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtAuthMiddleware {
            inner,
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtAuthMiddleware<S> {
    inner: S,
    jwt_secret: String,
}

impl<S> Service<Request> for JwtAuthMiddleware<S>
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

    fn call(&mut self, mut req: Request) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            match auth_header {
                Some(auth_header) => {
                    if let Some(token) = auth_header.strip_prefix("Bearer ") {
                        // Validate JWT token
                        match validate_access_token_claims(token, &jwt_secret) {
                            Ok(claims) => {
                                // Insert claims into request extensions for downstream middleware
                                req.extensions_mut().insert(claims);
                                // Continue with the request
                                inner.call(req).await
                            }
                            Err(_) => {
                                // Invalid token
                                let error = AuthError::InvalidToken;
                                Ok(error.into_response())
                            }
                        }
                    } else {
                        // Missing Bearer prefix
                        let error = AuthError::InvalidAuthFormat;
                        Ok(error.into_response())
                    }
                }
                None => {
                    // Missing Authorization header
                    let error = AuthError::MissingAuthHeader;
                    Ok(error.into_response())
                }
            }
        })
    }
}

#[derive(Debug)]
enum AuthError {
    MissingAuthHeader,
    InvalidAuthFormat,
    InvalidToken,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingAuthHeader => write!(f, "Missing Authorization header"),
            AuthError::InvalidAuthFormat => write!(f, "Invalid Authorization format"),
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingAuthHeader => {
                (StatusCode::UNAUTHORIZED, "Missing Authorization header")
            }
            AuthError::InvalidAuthFormat => {
                (StatusCode::UNAUTHORIZED, "Invalid Authorization format")
            }
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
        };

        let error_response = serde_json::json!({
            "success": false,
            "message": message,
            "error": self.to_string()
        });

        (status, Json(error_response)).into_response()
    }
}

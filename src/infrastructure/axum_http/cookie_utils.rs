use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

// Cookie configuration constants
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
pub const REFRESH_TOKEN_COOKIE_PATH: &str = "/";
pub const REFRESH_TOKEN_COOKIE_MAX_AGE: i64 = 7 * 24 * 60 * 60; // 7 days in seconds

#[derive(Debug, Clone)]
pub enum CookieError {
    InvalidHeaderValue(String),
    InvalidCookieFormat(String),
}

impl fmt::Display for CookieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CookieError::InvalidHeaderValue(msg) => write!(f, "Invalid header value: {}", msg),
            CookieError::InvalidCookieFormat(msg) => write!(f, "Invalid cookie format: {}", msg),
        }
    }
}

impl std::error::Error for CookieError {}

// Result type for cookie operations
pub type CookieResult<T> = Result<T, CookieError>;

pub fn set_refresh_token_cookie(response: Response, refresh_token: &str) -> Response {
    let cookie_value = format!(
        "{}={}; Path={}; Max-Age={}; HttpOnly; SameSite=Strict",
        REFRESH_TOKEN_COOKIE_NAME,
        refresh_token,
        REFRESH_TOKEN_COOKIE_PATH,
        REFRESH_TOKEN_COOKIE_MAX_AGE
    );

    let header_value = match HeaderValue::from_str(&cookie_value) {
        Ok(value) => value,
        Err(_) => return response, // Return original response if header creation fails
    };

    let (parts, body) = response.into_parts();
    let mut new_parts = parts;
    new_parts.headers.insert("Set-Cookie", header_value);

    Response::from_parts(new_parts, body)
}

// Clear refresh token cookie
pub fn clear_refresh_token_cookie(response: Response) -> Response {
    let cookie_value = format!(
        "{}={}; Path={}; Max-Age=0; HttpOnly; SameSite=Strict",
        REFRESH_TOKEN_COOKIE_NAME,
        "",
        REFRESH_TOKEN_COOKIE_PATH
    );

    let header_value = match HeaderValue::from_str(&cookie_value) {
        Ok(value) => value,
        Err(_) => return response, // Return original response if header creation fails
    };

    let (parts, body) = response.into_parts();
    let mut new_parts = parts;
    new_parts.headers.insert("Set-Cookie", header_value);

    Response::from_parts(new_parts, body)
}

// Extract refresh token from cookie
pub fn extract_refresh_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Cookie")
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str
                .split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with(&format!("{}=", REFRESH_TOKEN_COOKIE_NAME)) {
                        let token_start = REFRESH_TOKEN_COOKIE_NAME.len() + 1;
                        if cookie.len() > token_start {
                            Some(cookie[token_start..].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
        })
}

// Create a JSON error response for missing refresh token
pub fn missing_refresh_token_response() -> Response {
    let error_response = json!({
        "success": false,
        "message": "Refresh token not found in cookies",
        "error": "Missing refresh token"
    });

    (StatusCode::UNAUTHORIZED, axum::Json(error_response)).into_response()
}

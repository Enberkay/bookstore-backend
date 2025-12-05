// =============================================================================
// Axum HTTP Adapter Example
// =============================================================================
// To use this:
// 1. Uncomment Axum dependencies in Cargo.toml
// 2. Uncomment `pub mod axum_adapter;` in mod.rs
// 3. Implement your routes and handlers below
// =============================================================================

/*
use axum::{
    routing::{get, post},
    Router, Json,
    extract::State,
    http::StatusCode,
};
use std::sync::Arc;
use serde_json::json;

use crate::{
    application::use_cases::{
        auth_usecase::AuthUseCase,
        user_usecase::UserUseCase,
        role_usecase::RoleUseCase,
    },
    application::dtos::{
        auth_dto::{LoginRequest, RegisterRequest},
        user_dto::CreateUserRequest,
        role_dto::CreateRoleRequest,
    },
};

// =============================================================================
// Application State
// =============================================================================

#[derive(Clone)]
pub struct AppState {
    pub auth_usecase: Arc<AuthUseCase>,
    pub user_usecase: Arc<UserUseCase>,
    pub role_usecase: Arc<RoleUseCase>,
}

// =============================================================================
// Router Configuration
// =============================================================================

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_routes())
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/roles", role_routes())
}

// =============================================================================
// Auth Routes
// =============================================================================

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.auth_usecase.register(req).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.auth_usecase.login(req).await {
        Ok((response, _refresh_token)) => {
            // TODO: Set refresh_token in HTTP-only cookie
            Ok(Json(json!(response)))
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn refresh_token(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Extract refresh token from cookie
    // TODO: Call auth_usecase.refresh_token()
    Err(StatusCode::NOT_IMPLEMENTED)
}

// =============================================================================
// User Routes
// =============================================================================

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_all_users))
        .route("/:id", get(get_user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.user_usecase.create_user(req).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.user_usecase.get_all_users().await {
        Ok(users) => Ok(Json(json!(users))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Extract ID from path
    Err(StatusCode::NOT_IMPLEMENTED)
}

// =============================================================================
// Role Routes
// =============================================================================

fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_role))
        .route("/", get(get_all_roles))
}

async fn create_role(
    State(state): State<AppState>,
    Json(req): Json<CreateRoleRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.role_usecase.create_role(req).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn get_all_roles(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match state.role_usecase.get_all_roles().await {
        Ok(roles) => Ok(Json(json!(roles))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// =============================================================================
// Health Check
// =============================================================================

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "framework": "axum"
    }))
}
*/

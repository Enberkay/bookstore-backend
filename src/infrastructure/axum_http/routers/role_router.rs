use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use serde_json::json;
use validator::Validate;

use crate::application::{
    application_errors::ApplicationError,
    dtos::role_dto::{CreateRoleRequest, UpdateRoleRequest, RoleResponse},
    services::role_service::RoleService,
};

/// Build router for Role endpoints
pub fn routes(role_service: Arc<RoleService>) -> Router {
    Router::new()
        .route("/", post(create_role))
        .route("/", get(get_all_roles))
        .route("/{id}", get(get_role_by_id))
        .route("/{id}", put(update_role))
        .route("/{id}", delete(delete_role))
        .with_state(role_service)
}

/// POST /roles
async fn create_role(
    State(service): State<Arc<RoleService>>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>, ApplicationError> {
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    service.create_role(payload).await.map(Json).map_err(|e| {
        ApplicationError::internal(format!("Failed to create role: {}", e))
    })
}

/// GET /roles
async fn get_all_roles(
    State(service): State<Arc<RoleService>>,
) -> Result<Json<Vec<RoleResponse>>, ApplicationError> {
    service.get_all_roles().await.map(Json).map_err(|e| {
        ApplicationError::internal(format!("Failed to fetch roles: {}", e))
    })
}

/// GET /roles/{id}
async fn get_role_by_id(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
) -> Result<Json<RoleResponse>, ApplicationError> {
    match service.get_role_by_id(id).await.map_err(|e| {
        ApplicationError::internal(format!("Failed to fetch role: {}", e))
    })? {
        Some(role) => Ok(Json(role)),
        None => Err(ApplicationError::not_found("Role not found")),
    }
}

/// PUT /roles/{id}
async fn update_role(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<serde_json::Value>, ApplicationError> {
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    service.update_role(id, payload).await.map_err(|e| {
        ApplicationError::internal(format!("Failed to update role: {}", e))
    })?;

    Ok(Json(json!({ "status": "updated" })))
}

/// DELETE /roles/{id}
async fn delete_role(
    State(service): State<Arc<RoleService>>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ApplicationError> {
    service.delete_role(id).await.map_err(|e| {
        ApplicationError::internal(format!("Failed to delete role: {}", e))
    })?;

    Ok(Json(json!({ "status": "deleted" })))
}

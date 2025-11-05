use std::sync::Arc;
use axum::{
    extract::{Path, State},
    routing::{get, post, put, delete},
    Json, Router,
};

use validator::Validate;

use crate::application::{
    application_errors::ApplicationError,
    dtos::permission_dto::{CreatePermissionRequest, UpdatePermissionRequest, PermissionResponse},
    services::permission_service::PermissionService,
};

/// Build router for Permission endpoints
pub fn routes(permission_service: Arc<PermissionService>) -> Router {
    Router::new()
        .route("/", post(create_permission))
        .route("/", get(get_all_permissions))
        .route("/{id}", get(get_permission_by_id))
        .route("/{id}", put(update_permission))
        .route("/{id}", delete(delete_permission))
        .with_state(permission_service)
}

/// POST /permissions
async fn create_permission(
    State(service): State<Arc<PermissionService>>,
    Json(payload): Json<CreatePermissionRequest>,
) -> Result<Json<PermissionResponse>, ApplicationError> {
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    service.create_permission(payload).await.map(Json)
}

/// GET /permissions
async fn get_all_permissions(
    State(service): State<Arc<PermissionService>>,
) -> Result<Json<Vec<PermissionResponse>>, ApplicationError> {
    service.get_all_permissions().await.map(Json)
}

/// GET /permissions/{id}
async fn get_permission_by_id(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, ApplicationError> {
    match service.get_permission_by_id(id).await? {
        Some(perm) => Ok(Json(perm)),
        None => Err(ApplicationError::not_found("Permission not found")),
    }
}

/// PUT /permissions/{id}
async fn update_permission(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePermissionRequest>,
) -> Result<Json<PermissionResponse>, ApplicationError> {
    payload
        .validate()
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

    // Update และคืนข้อมูลที่อัพเดตแล้วในครั้งเดียว
    Ok(Json(service.update_permission(id, payload).await?))
}

/// DELETE /permissions/{id}
async fn delete_permission(
    State(service): State<Arc<PermissionService>>,
    Path(id): Path<i32>,
) -> Result<Json<PermissionResponse>, ApplicationError> {
    // Delete and return deleted permission data
    Ok(Json(service.delete_permission(id).await?))
}

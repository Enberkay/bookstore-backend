// =============================================================================
// Actix Web HTTP Adapter Example
// =============================================================================
// To use this:
// 1. Uncomment Actix Web dependencies in Cargo.toml
// 2. Uncomment `pub mod actix_adapter;` in mod.rs
// 3. Implement your routes and handlers below
// =============================================================================

/*
use actix_web::{
    web::{self, Data, Json, Path},
    HttpResponse, Responder,
    middleware::Logger,
};
use actix_cors::Cors;
use std::sync::Arc;

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

pub struct AppState {
    pub auth_usecase: Arc<AuthUseCase>,
    pub user_usecase: Arc<UserUseCase>,
    pub role_usecase: Arc<RoleUseCase>,
}

// =============================================================================
// Server Configuration
// =============================================================================

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(health_check)
            .service(auth_routes())
            .service(user_routes())
            .service(role_routes())
    );
}

pub fn configure_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
            actix_web::http::header::CONTENT_TYPE,
        ])
        .supports_credentials()
        .max_age(3600)
}

// =============================================================================
// Auth Routes
// =============================================================================

fn auth_routes() -> actix_web::Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/refresh", web::post().to(refresh_token))
}

async fn register(
    state: Data<AppState>,
    req: Json<RegisterRequest>,
) -> impl Responder {
    match state.auth_usecase.register(req.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn login(
    state: Data<AppState>,
    req: Json<LoginRequest>,
) -> impl Responder {
    match state.auth_usecase.login(req.into_inner()).await {
        Ok((response, _refresh_token)) => {
            // TODO: Set refresh_token in HTTP-only cookie
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn refresh_token(
    _state: Data<AppState>,
) -> impl Responder {
    // TODO: Extract refresh token from cookie
    // TODO: Call auth_usecase.refresh_token()
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented"
    }))
}

// =============================================================================
// User Routes
// =============================================================================

fn user_routes() -> actix_web::Scope {
    web::scope("/users")
        .route("", web::post().to(create_user))
        .route("", web::get().to(get_all_users))
        .route("/{id}", web::get().to(get_user))
        .route("/{id}", web::put().to(update_user))
        .route("/{id}", web::delete().to(delete_user))
}

async fn create_user(
    state: Data<AppState>,
    req: Json<CreateUserRequest>,
) -> impl Responder {
    match state.user_usecase.create_user(req.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn get_all_users(
    state: Data<AppState>,
) -> impl Responder {
    match state.user_usecase.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn get_user(
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match state.user_usecase.get_user_by_id(id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn update_user(
    _state: Data<AppState>,
    _path: Path<i32>,
) -> impl Responder {
    // TODO: Implement update logic
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented"
    }))
}

async fn delete_user(
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match state.user_usecase.delete_user(id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

// =============================================================================
// Role Routes
// =============================================================================

fn role_routes() -> actix_web::Scope {
    web::scope("/roles")
        .route("", web::post().to(create_role))
        .route("", web::get().to(get_all_roles))
        .route("/{id}", web::get().to(get_role))
        .route("/{id}", web::put().to(update_role))
        .route("/{id}", web::delete().to(delete_role))
}

async fn create_role(
    state: Data<AppState>,
    req: Json<CreateRoleRequest>,
) -> impl Responder {
    match state.role_usecase.create_role(req.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn get_all_roles(
    state: Data<AppState>,
) -> impl Responder {
    match state.role_usecase.get_all_roles().await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn get_role(
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match state.role_usecase.get_role_by_id(id).await {
        Ok(Some(role)) => HttpResponse::Ok().json(role),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Role not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

async fn update_role(
    _state: Data<AppState>,
    _path: Path<i32>,
) -> impl Responder {
    // TODO: Implement update logic
    HttpResponse::NotImplemented().json(serde_json::json!({
        "error": "Not implemented"
    }))
}

async fn delete_role(
    state: Data<AppState>,
    path: Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match state.role_usecase.delete_role(id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

// =============================================================================
// Health Check
// =============================================================================

#[actix_web::get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "framework": "actix-web"
    }))
}

// =============================================================================
// Example: How to start the server in main.rs
// =============================================================================

/*
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize your use cases and repositories here
    let app_state = web::Data::new(AppState {
        auth_usecase: Arc::new(auth_usecase),
        user_usecase: Arc::new(user_usecase),
        role_usecase: Arc::new(role_usecase),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .wrap(configure_cors())
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
*/
*/

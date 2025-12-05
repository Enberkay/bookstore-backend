// =============================================================================
// Clean Architecture + DDD Template - Main Entry Point
// =============================================================================
// This is a minimal example showing how to bootstrap your application.
// For full HTTP server examples, see:
//   - examples/axum_server.rs (for Axum framework)
//   - examples/actix_server.rs (for Actix Web framework)
// =============================================================================

use clean_architecture_template::{
    infrastructure::config,
    adapters::postgres::postgres_connector,
};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // 1. Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Warning: couldn't load .env file: {}", e);
    }

    // 2. Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init();

    // 3. Load and validate application configuration
    let app_config = match config::load() {
        Ok(cfg) => {
            info!("Configuration loaded (environment: {:?})", cfg.environment);
            cfg
        }
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    // 4. Establish database connection pool
    let _pg_pool = match postgres_connector::establish_connection(&app_config.database.url).await {
        Ok(pool) => {
            info!("PostgreSQL connection pool established");
            pool
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {:?}", e);
            std::process::exit(1);
        }
    };

    // 5. TODO: Start your HTTP server here
    // Uncomment one of the following based on your chosen framework:

    // For Axum:
    // See examples/axum_server.rs for complete implementation

    // For Actix Web:
    // See examples/actix_server.rs for complete implementation

    info!("Application initialized successfully");
    info!("This is a template - add your HTTP server implementation!");
    info!(" See examples/ directory for Axum and Actix Web implementations");
}

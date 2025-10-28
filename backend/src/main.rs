use axum::{
    routing::{get, post, put, delete},
    Router,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use sqlx::sqlite::SqlitePool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod models;
mod routes;
mod db;
mod scraper;

use config::Config;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "realestate_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Set up database connection pool
    let pool = SqlitePool::connect(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    
    tracing::info!("Database migrations completed");

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(config.cors_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build application routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api", api_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Real Estate Dashboard API - v0.1.0"
}

async fn health_check() -> &'static str {
    "OK"
}

fn api_routes() -> Router<SqlitePool> {
    Router::new()
        // Property routes
        .route("/properties", get(routes::properties::list_properties))
        .route("/properties", post(routes::properties::create_property))
        .route("/properties/:id", get(routes::properties::get_property))
        .route("/properties/:id", put(routes::properties::update_property))
        .route("/properties/:id", delete(routes::properties::delete_property))
        
        // Tenant routes
        .route("/tenants", get(routes::tenants::list_tenants))
        .route("/tenants", post(routes::tenants::create_tenant))
        .route("/tenants/:id", get(routes::tenants::get_tenant))
        .route("/tenants/:id", put(routes::tenants::update_tenant))
        .route("/tenants/:id", delete(routes::tenants::delete_tenant))
        
        // Calendar/Events routes
        .route("/events", get(routes::events::list_events))
        .route("/events", post(routes::events::create_event))
        .route("/events/:id", get(routes::events::get_event))
        .route("/events/:id", put(routes::events::update_event))
        .route("/events/:id", delete(routes::events::delete_event))
        
        // Maintenance routes
        .route("/maintenance", get(routes::maintenance::list_maintenance))
        .route("/maintenance", post(routes::maintenance::create_maintenance))
        .route("/maintenance/:id", put(routes::maintenance::update_maintenance))
        
        // Market data routes
        .route("/market/trends", get(routes::market::get_trends))
        .route("/market/analytics", get(routes::market::get_analytics))
        .route("/market/scrape", post(routes::market::trigger_scrape))
}

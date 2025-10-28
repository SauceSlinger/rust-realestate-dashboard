mod models;
mod db;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = db::init_db().await.expect("Failed to initialize database");

    let app = Router::new()
        // Property routes
        .route("/api/properties", get(handlers::properties::list_properties))
        .route("/api/properties", post(handlers::properties::create_property))
        .route("/api/properties/:id", get(handlers::properties::get_property))
        .route("/api/properties/:id", put(handlers::properties::update_property))
        .route("/api/properties/:id", delete(handlers::properties::delete_property))
        // Reminder routes
        .route("/api/reminders", get(handlers::reminders::list_reminders))
        .route("/api/reminders", post(handlers::reminders::create_reminder))
        .route("/api/reminders/:id", put(handlers::reminders::update_reminder))
        .route("/api/reminders/:id", delete(handlers::reminders::delete_reminder))
        // Market data routes
        .route("/api/market-data", get(handlers::market_data::list_market_data))
        .route("/api/market-data", post(handlers::market_data::create_market_data))
        .route("/api/market-data/trends", get(handlers::market_data::get_trends))
        .route("/api/scrape", post(handlers::scraper::scrape_market_data))
        // Tenant routes
        .route("/api/tenants", get(handlers::tenants::list_tenants))
        .route("/api/tenants", post(handlers::tenants::create_tenant))
        .route("/api/tenants/:id", put(handlers::tenants::update_tenant))
        .route("/api/tenants/:id", delete(handlers::tenants::delete_tenant))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

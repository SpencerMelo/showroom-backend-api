use axum::{Router};
use axum::http::Method;
use dotenvy::dotenv;
use log::{error, info};
use tower_http::cors::CorsLayer;
use tower_http::cors::Any;
use tokio::signal;
use crate::database::database::get_connection_pool;

use crate::resource::post_controller;

mod resource;
mod schema;
mod service;
mod database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Establishing database connection pool...");
    let pool = get_connection_pool().unwrap_or_else(|_| {
        panic!("Unable to establish database connection")
    });
    info!("Database connection pool established.");

    info!("Establishing server configurations");
    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PATCH]).allow_origin(Any);
    let routes = Router::new().merge(post_controller::router(pool).layer(cors));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server configurations established.");

    info!("Starting server...");
    axum::serve(listener, routes)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|err| {
            error!("Unable to start the server, error: {}", err);
        });
    info!("Server stopped.");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown");
}
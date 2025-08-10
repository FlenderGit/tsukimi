use axum::http::Method;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

pub mod config;
pub mod routes;
pub mod services;

#[derive(Clone)]
struct AppState {
    pub database: services::database::DatabaseService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let version = env!("CARGO_PKG_VERSION");
    let app_name = env!("CARGO_PKG_NAME");

    // Load configuration
    let config = config::get_configuration().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    info!("{} v{}", app_name, version);
    info!("Running in {:?} mode", config.env());
    #[cfg(debug_assertions)]
    info!("/!\\ Debug mode is enabled");

    let (database_service) = services::get_services(&config).await.map_err(|e| {
        error!("Failed to initialize services: {}", e);
        e
    })?;

    let app_state = AppState {
        database: database_service,
    };

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port()));
    let listener = TcpListener::bind(address).await.map_err(|e| {
        error!("Failed to bind to address {}: {}", address, e);
        e
    })?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let router = routes::get_router().with_state(app_state).layer(cors);

    info!("Starting Tsukimi CDN on port: {}", config.port());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|e| {
            error!("Failed to start server: {}", e);
            e
        })?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = terminate => {
            info!("Received termination signal, shutting down...");
        }
    }
}

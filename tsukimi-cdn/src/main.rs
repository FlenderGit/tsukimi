use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::{error, info};

pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        // .with_env_filter("info")
        // .with_target(false)
        // .with_thread_ids(true)
        // .with_thread_names(true)
        .init();
    let config = config::get_configuration().map_err(|e| {
        error!("Failed to load configuration: {}", e);
        e
    })?;

    info!("Starting Tsukimi CDN on port: {}", config.port());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, config.port()));

    let listener = TcpListener::bind(address).await.map_err(|e| {
        error!("Failed to bind to address {}: {}", address, e);
        e
    })?;

    axum::serve(listener, axum::Router::new())
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

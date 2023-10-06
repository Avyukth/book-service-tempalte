mod db;
mod rest;
use crate::db::init_db;
use std::env;
use std::net::{IpAddr, SocketAddr};
use tokio::signal;

use anyhow::{Ok, Result};
use axum::{Extension, Router};
use sqlx::SqlitePool;

fn router(pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/books", rest::book_service())
        // .nest_service("/")
        .layer(Extension(pool))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let connection_pool = init_db().await?;

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8090".to_string())
        .parse()?;

    let app = router(connection_pool);

    let addr = SocketAddr::from((host.parse::<IpAddr>()?, port));

    // Create a future that listens for shutdown signal
    let shutdown_signal = shutdown_signal();

    println!("Server running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    println!("Server shutting down");

    Ok(())
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

    println!("signal received, starting graceful shutdown");
}

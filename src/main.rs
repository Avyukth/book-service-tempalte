mod db;
mod init_trace;
mod rest;
use crate::db::init_db;
use anyhow::{Ok, Result};
use axum::{Extension, Router};
use sqlx::SqlitePool;
use std::env;
use std::net::{IpAddr, SocketAddr};
use tokio::signal;
use tower_http::trace::{self, TraceLayer};

fn router(pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/books", rest::book_service())
        // .nest_service("/")
        .layer(Extension(pool))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    init_trace::init_tracing();
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

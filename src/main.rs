mod db;
mod rest;
use crate::db::init_db;
use std::env;
use std::net::{IpAddr, SocketAddr};

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
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

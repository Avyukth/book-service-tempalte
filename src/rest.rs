use crate::db::{add_book, delete_book, get_all_books, get_book_by_id, update_book};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::SqlitePool;

pub fn book_service() -> Router {
    Router::new()
        .get("/books", get_all_books)
        .get("/books/:id", get_book_by_id)
        .post("/books", add_book)
        .put("/books/:id", update_book)
        .delete("/books/:id", delete_book)
}

use crate::db::Book;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::SqlitePool;

pub fn book_service() -> Router {
    Router::new()
        .route("/", get(get_books))
        .route("/:id", get(get_book))
        .route("/add", post(add_book))
        .route("/edit", put(update_book))
        .route("/delete/:id", delete(delete_book))
}

async fn get_books(Extension(pool): Extension<SqlitePool>) -> Result<Json<Vec<Book>>, StatusCode> {
    match crate::db::get_all_books(&pool).await {
        Ok(books) => Ok(Json(books)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn get_book(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    match crate::db::get_book_by_id(&pool, id).await {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn add_book(
    Extension(pool): Extension<SqlitePool>,
    extract::Json(book): Json<Book>,
) -> Result<Json<i32>, StatusCode> {
    match crate::db::add_book(&pool, &book.title, &book.author).await {
        Ok(id) => Ok(Json(id)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn update_book(
    Extension(pool): Extension<SqlitePool>,
    extract::Json(book): extract::Json<Book>,
) -> StatusCode {
    match crate::db::update_book(&pool, &book).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

async fn delete_book(
    Extension(pool): Extension<SqlitePool>,
    extract::Path(id): Path<i32>,
) -> StatusCode {
    match crate::db::delete_book(&pool, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum_test_helper::TestClient;

    async fn setup_tests() -> TestClient {
        dotenv::dotenv().ok();
        let connection_pool = crate::db::init_db().await.unwrap();
        let app = crate::router(connection_pool);
        TestClient::new(app)
    }

    #[tokio::test]
    async fn get_books() {
        let client = setup_tests().await;
        let response = client.get("/books").send().await;
        assert_eq!(response.status(), StatusCode::OK);
        let books = response.json::<Vec<Book>>().await;
        assert!(!books.is_empty());
    }
}

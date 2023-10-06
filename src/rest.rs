use crate::db::{add_book, delete_book, get_all_books, get_book_by_id, update_book};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, Extension, Json, Router};
use sqlx::SqlitePool;

pub fn book_service() -> Router {
    Router::new()
        .route("/", get(get_all_books))
        .route("/:id", get(get_book))
        .route("/add", post(add_book))
        .route("/edit", put(update_book))
        .route("/delete/:id", delete(delete_book))
}

async fn get_all_books(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Book>>, StatusCode> {
    match db::get_all_books(&pool).await {
        Ok(books) => Ok(Json(books)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn get_book(
    Extension(pool): Extension<SqlitePool>,
    path(id): Path<i32>,
) -> Result<Json<Book>, StatusCode> {
    match db::get_book_by_id(&pool, id).await {
        Ok(book) => Ok(Json(book)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn add_book(
    Extension(pool): Extension<SqlitePool>,
    extract::JSON(book): Json<Book>,
) -> Result<Json<i32>, StatusCode> {
    match db::add_book(&pool, &book.title, &book.author).await {
        Ok(id) => Ok(Json(id)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn update_book(
    Extension(pool): Extension<SqlitePool>,
    extract::JSON(book): Json<Book>,
) -> StatusCode {
    match db::update_book(&pool, &book).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

async fn delete_book(
    Extension(pool): Extension<SqlitePool>,
    extract::Path(id): Path<i32>,
) -> StatusCode {
    match db::delete_book(&pool, id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum_test_helper::TestClient;

    async fn setup_tests() -> TestClient {
        dotenv::dotenv().ok();
        let connection_pool = init_db().await.unwrap();
        let app = crate::router(connection_pool);
        TestClient::new(app)
    }

    #[tokio::test]
    async fn get_all_books() {
        let client = setup_tests().await;
        let response = client.get("/books").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let books = response.json::<Vec<Book>>().await.unwrap();
        assert!(!books.is_empty());
    }
}

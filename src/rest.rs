use crate::db::Book;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get, post, put};
use axum::{extract, response::IntoResponse, Extension, Json, Router};
use serde_json::json;
use sqlx::SqlitePool;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

pub fn book_service() -> Router {
    Router::new()
        .route(
            "/proxy/:service/*path",
            get(proxy_handler).post(proxy_handler),
        )
        .route("/", get(index))
        .route("/all", get(get_books))
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

#[tracing::instrument]
async fn index() -> impl IntoResponse {
    let trace_id = find_current_trace_id();
    dbg!(&trace_id);
    //std::thread::sleep(std::time::Duration::from_secs(1));
    axum::Json(json!({ "my_trace_id": trace_id }))
}

async fn proxy_handler(Path((service, path)): Path<(String, String)>) -> impl IntoResponse {
    // Overwrite the otel.name of the span
    tracing::Span::current().record("otel.name", format!("proxy {service}"));
    let trace_id = find_current_trace_id();
    axum::Json(
        json!({ "my_trace_id": trace_id, "fake_proxy": { "service": service, "path": path } }),
    )
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

    #[tokio::test]
    async fn get_one_book() {
        let client = setup_tests().await;
        let response = client.get("/books/1").send().await;
        assert_eq!(response.status(), StatusCode::OK);
        let book = response.json::<Book>().await;
        assert_eq!(book.id, 1);
        assert_eq!("Hands-on Rust", book.title);
        assert_eq!("Wolverson, Herbert", book.author);
    }

    #[tokio::test]
    async fn add_book() {
        let client = setup_tests().await;
        let new_book = Book {
            id: -1,
            title: "Test POST Book".to_string(),
            author: "Test POST Author".to_string(),
        };
        let response = client.post("/books/add").json(&new_book).send().await;
        assert_eq!(response.status(), StatusCode::OK);
        let new_id: i32 = response.json().await;
        assert!(new_id > 0);

        let test_book = client.get(&format!("/books/{}", new_id)).send().await;
        assert_eq!(test_book.status(), StatusCode::OK);
        let test_book = test_book.json::<Book>().await;
        assert_eq!(test_book.id, new_id);
        assert_eq!(new_book.title, test_book.title);
        assert_eq!(new_book.author, test_book.author);
    }

    #[tokio::test]
    async fn update_book() {
        let client = setup_tests().await;
        let mut book1: Book = client.get("/books/1").send().await.json().await;
        book1.title = "Updated book".to_string();
        let res = client.put("/books/edit").json(&book1).send().await;
        assert_eq!(res.status(), StatusCode::OK);
        let book2: Book = client.get("/books/1").send().await.json().await;
        assert_eq!(book1.title, book2.title);
    }

    #[tokio::test]
    async fn delete_book() {
        let client = setup_tests().await;
        let new_book = Book {
            id: -1,
            title: "Delete me".to_string(),
            author: "Delete me".to_string(),
        };
        let new_id: i32 = client
            .post("/books/add")
            .json(&new_book)
            .send()
            .await
            .json()
            .await;

        let res = client
            .delete(&format!("/books/delete/{new_id}"))
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::OK);

        let all_books: Vec<Book> = client.get("/books").send().await.json().await;
        assert!(all_books.iter().find(|b| b.id == new_id).is_none())
    }
}

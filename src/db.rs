use std::sync::RwLock;

use anyhow::{Result, Ok};
use sqlx::{SqlitePool, FromRow, Row};




#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Book{

    pub id: i32,
    pub title   : String,
    pub author  : String,
}


struct BookCache{
    all_books: RwLock<Option<Vec<Book>>>,
}

pub async fn init_db() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be present in environment variables");
    let connection_pool = SqlitePool::connect(&database_url).await?;
    Ok(connection_pool)
}

pub async fn get_all_books(connection_pool:&SqlitePool) -> Result<Vec<Book>> {
    Ok(
        sqlx::query_as::<_, Book>("SELECT * FROM books order by title, author")
      .fetch_all(connection_pool)
      .await?,
    
    )
}

pub async fn get_book_by_id(connection_pool:&SqlitePool, id:i32) -> Result<Book> {
    Ok(

        sqlx::query_as::<_, Book>("SELECT * FROM books where id = $1")
        .bind(id)
        .fetch_one(connection_pool)
        .await?,
    )
}


pub async fn add_book<S:ToString>(connection_pool:&SqlitePool, title:S , author:S) -> Result<i32> {


    let title = title.to_string();
    let author = author.to_string();
    Ok(
    sqlx::query("INSERT INTO books (title, author) VALUES ($1, $2) RETURNING id")
      .bind(title)
      .bind(author)
      .fetch_one(connection_pool)
      .await?
      .get(0),
    )
}

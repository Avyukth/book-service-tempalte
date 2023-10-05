mod db;
use crate::db::init_db;

use anyhow::{Result, Ok};


#[tokio::main]
async fn main()-> Result<()> {

    dotenv ::dotenv().ok();
    let connection_pool = init_db().await?;
    Ok(())
}

#[allow(unused_imports)]
use crate::models::shortcut::Shortcut;

use std::{env, str::FromStr};
use sqlx::{sqlite::{SqlitePool, SqliteConnectOptions}, Pool, Sqlite};
use tracing::info;

pub async fn connect_db() -> Pool<Sqlite> {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_FILENAME not set");
  let options = SqliteConnectOptions::from_str(&database_url).unwrap();

  let pool: SqlitePool = SqlitePool::connect_with(options).await.expect("DB connection failed");
  info!("Database connection made.");

  let _ = sqlx::migrate!("./migrations").run(&pool).await; 
  info!("Migrations complete!");

  // let google: Shortcut = Shortcut::new("google".to_owned(), "https://google.co.uk".to_owned());

  // sqlx::query("INSERT OR IGNORE INTO shortcuts (created, updated, keyword, url) VALUES (?1, ?2, ?3, ?4)")
  // .bind(google.created)
  // .bind(google.updated)
  // .bind(google.keyword)
  // .bind(google.url)
  // .execute(&pool).await.expect("DB new entry failed");
  // info!("Successfully added DB entry.");

  pool
}
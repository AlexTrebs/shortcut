use crate::{error::ShortcutError, models::shortcut::Shortcut};

use rust_fuzzy_search::fuzzy_compare;
use sqlx::{Error, Pool, Sqlite};
use tracing::{debug, error};

#[derive(Clone)]
pub struct ShortcutRepository {
  database: Pool<Sqlite>,
}

pub trait ShortcutRepositoryTrait {
  fn new(database: Pool<Sqlite>) -> Self;
  async fn fuzzy_search(&self, search: &str) -> Result<Vec<Shortcut>, ShortcutError>;
  async fn get(&self, keyword: &String) -> Result<Shortcut, Error>;
  async fn create(&self, todo: &Shortcut) -> Result<bool, Error>;
  async fn update(&self, todo: &Shortcut) -> Result<bool, ShortcutError>;
}

impl ShortcutRepositoryTrait for ShortcutRepository {
  fn new(database: Pool<Sqlite>) -> Self {
    ShortcutRepository { database }
  }

  async fn fuzzy_search(&self, search: &str) -> Result<Vec<Shortcut>, ShortcutError> {
    let result = sqlx::query_as!(Shortcut, r#"SELECT * FROM shortcuts;"#)
      .fetch_all(&self.database)
      .await;

    match result {
      Ok(shortcuts) => {
        let mut matches:Vec<(f32, Shortcut)> = shortcuts.into_iter()
          .filter_map(|shortcut| {
            let score = fuzzy_compare(search, &shortcut.keyword);
            if score > 0.0 {
              Some((score, shortcut))  // Keep only matching results
            } else {
              None  // Skip non-matching results
            }
          })
          .collect();

        matches.sort_by(|a: &(f32, Shortcut), b| b.0.partial_cmp(&a.0).unwrap());

        let matches: Vec<Shortcut> = matches.into_iter().map(|(_, shortcut)| shortcut).collect();

        debug!("Found {:?} matches.", matches.len());
        Ok(matches)
      },
      Err(err) => {
        error!("Failed to find similar shortcuts from database: {}", err);
        Err(ShortcutError::FailedToGet)
      }
    }
  }

  async fn get(&self, keyword: &String) -> Result<Shortcut, Error> {
    let result = sqlx::query_as!(Shortcut, r#"SELECT * FROM shortcuts WHERE keyword = ?1;"#, keyword)
      .fetch_one(&self.database).await;

    match result {
      Ok(result) => Ok(result),
      Err(err) => {
        error!("Failed to get shortcut for ({}) from database: {}", keyword, err);
        Err(err)
      }
    }
  }

  async fn create(&self, shortcut: &Shortcut) -> Result<bool, Error> {
    let result = sqlx::query!(
      r#"INSERT INTO shortcuts (created, updated, keyword, url) VALUES (?1, ?2, ?3, ?4)"#, 
      shortcut.created, 
      shortcut.updated,
      shortcut.keyword,
      shortcut.url
    )
    .execute(&self.database)
    .await;

    match result {
      Ok(_) => Ok(true),
      Err(err) => {
        error!("Failed to add shortcut ({}) to database: {}", shortcut.keyword, err);
        Err(err)
      }
    }
  }

  async fn update(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError> {
    let result = sqlx::query!(
      r#"UPDATE shortcuts SET updated = ?1, url = ?2 WHERE keyword = ?3"#, 
      shortcut.updated,
      shortcut.url,
      shortcut.keyword
    )
    .execute(&self.database)
    .await;

    match result {
      Ok(_) => Ok(true),
      Err(err) => {
        error!("Failed to update shortcut ({}) in database: {}", shortcut.keyword, err);
        Err(ShortcutError::FailedToUpdate)
      }
    }
  }
}

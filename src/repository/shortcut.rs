use crate::{error::ShortcutError, models::shortcut::Shortcut};

#[allow(unused_imports)]
use mockall::{predicate::*, automock};
use rust_fuzzy_search::fuzzy_compare;
use sqlx::{Error, Pool, Sqlite};
use tracing::{debug, error};

/// This repository sits between the SQLite db and the service interactions.
/// 
/// We have 5 total functions:
/// - A constructor requiring a SQLite connection pool (`Pool<Sqlite>`),
/// - A fuzzy seach funtion to get close results, 
/// - A get method for exact `Shortcut` match by keyword,
/// - A create method to insert a `Shortcut` into the db, 
/// - And a update method to change the url of a `Shortcut`.
#[derive(Clone)]
pub struct ShortcutRepository {
  pub database: Pool<Sqlite>,
}

#[cfg_attr(test, automock)]
pub trait ShortcutRepositoryTrait {
  /// Creates a new instance of `ShortcutRepository`.
  fn new(database: Pool<Sqlite>) -> Self;

  /// A function to get the closest results for the given input using `rust_fuzzy_search`. 
  /// 
  /// This will return a `Vec` of `Shortcut` objects, sorted by the closest match of keyword to input.
  /// 
  /// It will remove all the object that did not match the search term at all.
  /// 
  /// ## Parameters
  /// - `search`: The unique `String` keyword to search.
  /// 
  /// ## Returns
  /// - `Result<Vec<Shortcut>, ShortcutError>`, will a `Vec<Shortcut>` sorted by how similar to the search term if there are matches, `ShortcutError::NotFound`
  ///   if there are no matching `Shortcut`'s, or `ShortcutError::FailedToSearch` if error occurs while retrieving.
  async fn fuzzy_search(&self, search: &str) -> Result<Vec<Shortcut>, ShortcutError>;

  /// A function to get the `Shortcut` object given an inputed keyword.
  /// 
  /// ## Parameters
  /// - `keyword`: The unique `String` keyword for teh Shortcut required.
  /// 
  /// ## Returns
  /// - `Result<Shortcut, ShortcutError>`, will be the requested `Shortcut` if exists, `ShortcutError::NotFound` if the `Shortcut` doesn't exists,
  ///   or `ShortcutError::FailedToGet` if error occurs while retrieving.
  async fn get(&self, keyword: &str) -> Result<Shortcut, ShortcutError>;

  /// A function to create a `Shortcut` given an input `Shortcut` object.
  /// 
  /// This will take the `created`, `updated`, `url` and `keyword` fields from the `Shortcut` object and insert them into the database.
  /// 
  /// You can expect this to return `true` if successfully created.
  /// 
  /// ## Parameters
  /// - `shortcut`: The `&Shortcut` to create.
  /// 
  /// ## Returns
  /// - `Result<bool, ShortcutError>`, will be true if successful, `ShortcutError::UniqueConstraintError` if the `Shortcut` already exists,
  ///   or `ShortcutError::FailedToCreate` if error occurs while creating.
  async fn create(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError>;

  /// A function to update a `Shortcut` given an inputed shortcut object.
  /// 
  /// This will take the `created`, and `url` fields from the `Shortcut` object and update the `Shortcut` with the same `keyword`.
  /// 
  /// You can expect this to return `true` if successfully updated.
  /// 
  /// ## Parameters
  /// - `shortcut`: The `&Shortcut` to update.
  /// 
  /// ## Returns
  /// - `Result<bool, ShortcutError>`, will be true if successful, false if nothing is updated, and `ShortcutError::FailedToUpdare`
  ///   if an error occurs while updating.
  async fn update(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError>;

  /// A function to delete a `Shortcut` given an inputed keyword.
  /// 
  /// This will take the `keyword`, from a `Shortcut` object and delete the `Shortcut` with the same `keyword`.
  /// 
  /// You can expect this to return `true` if successfully deletes.
  /// 
  /// ## Parameters
  /// - `shortcut`: The `&Shortcut` to update.
  /// 
  /// ## Returns
  /// - `Result<bool, ShortcutError>`, will be true if successful, false if nothing is deleted, and `ShortcutError::FailedToUpdare`
  ///   if an error occurs while updating.
  async fn delete(&self, keyword: &str) -> Result<bool, ShortcutError>;
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

        if matches.is_empty() {
          return Err(ShortcutError::NoMatches);
        }

        matches.sort_by(|a: &(f32, Shortcut), b| b.0.partial_cmp(&a.0).unwrap());

        let matches: Vec<Shortcut> = matches.into_iter().map(|(_, shortcut)| shortcut).collect();

        debug!("Found {:?} matches.", matches.len());
        Ok(matches)
      },
      Err(err) => {
        error!("Failed to find similar shortcuts from database: {}", err);
        Err(ShortcutError::FailedToSearch)
      }
    }
  }

  async fn get(&self, keyword: &str) -> Result<Shortcut, ShortcutError> {
    let result = sqlx::query_as!(Shortcut, r#"SELECT * FROM shortcuts WHERE keyword = ?1;"#, keyword)
      .fetch_one(&self.database).await;

    match result {
      Ok(result) => Ok(result),
      Err(Error::RowNotFound) => Err(ShortcutError::NotFound),
      Err(err) => {
        error!("Failed to get shortcut for ({}) from database: {}", keyword, err);
        Err(ShortcutError::FailedToGet)
      }
    }
  }

  async fn create(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError> {
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
      Err(Error::Database(db_err)) if db_err.message().contains("UNIQUE constraint failed") => Err(ShortcutError::UniqueConstraintError),
      Err(err) => {
        error!("Failed to add shortcut ({}) to database: {}", shortcut.keyword, err);
        Err(ShortcutError::FailedToCreate)
      }
    }
  }

  async fn update(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError> {
    let result = sqlx::query!(
      r#"UPDATE shortcuts SET url = ?1, updated = ?2 WHERE keyword = ?3"#, 
      shortcut.url,
      shortcut.updated,
      shortcut.keyword
    )
    .execute(&self.database)
    .await;

    match result {
      Ok(res) => 
        if res.rows_affected() == 1 {Ok(true)} 
        else if res.rows_affected() == 0 {Ok(false)}
        else {
          error!("Failed to update shortcut ({}) in database: updated {} rows", shortcut.keyword, res.rows_affected());

          Err(ShortcutError::FailedToUpdate)
        },
      Err(err) => {
        error!("Failed to update shortcut ({}) in database: {}", shortcut.keyword, err);
        Err(ShortcutError::FailedToUpdate)
      }
    }
  }

  async fn delete(&self, keyword: &str) -> Result<bool, ShortcutError> {
    let result = sqlx::query!(r#"DELETE from shortcuts WHERE keyword = ?1"#, keyword)
      .execute(&self.database)
      .await;

    match result {
      Ok(res) => 
        if res.rows_affected() == 1 {Ok(true)} 
        else if res.rows_affected() == 0 {Ok(false)}
        else {
          error!("Failed to delete shortcut ({}) in database: updated {} rows", keyword, res.rows_affected());

          Err(ShortcutError::FailedToDelete)
        },
      Err(err) => {
        error!("Failed to update shortcut ({}) in database: {}", keyword, err);
        Err(ShortcutError::FailedToDelete)
      }
    }
  }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[cfg(test)]
mod shortcut_repository_tests {
  use sqlx::{Executor, Pool, Sqlite, SqlitePool};

  use crate::{models::shortcut::Shortcut, repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait}};
  use lazy_static::lazy_static;

  lazy_static! {
    pub static ref BING_SHORTCUT: Shortcut = Shortcut { 
        id: Some(5),
        created: Some(3), 
        updated: Some(4), 
        keyword: "bing".to_owned(),
        url: "https://bing.co.uk".to_owned(),
      };

    pub static ref GOOGLE_SHORTCUT: Shortcut = Shortcut { 
        id: Some(1),
        created: Some(1), 
        updated: Some(2), 
        keyword: "google".to_owned(),
        url: "https://google.co.uk".to_owned(),
      };

    pub static ref G_SHORTCUT: Shortcut = Shortcut { 
        id: Some(2),
        created: Some(11), 
        updated: Some(22), 
        keyword: "g".to_owned(),
        url: "https://google.com".to_owned(),
      };

    pub static ref GLE_SHORTCUT: Shortcut = Shortcut { 
        id: Some(4),
        created: Some(1111), 
        updated: Some(2222), 
        keyword: "gle".to_owned(),
        url: "https://greatlightexchange.co.uk".to_owned(),
      };

    pub static ref GOOGLE_COM_SHORTCUT: Shortcut = Shortcut { 
        id: Some(1),
        created: Some(1), 
        updated: Some(3), 
        keyword: "google".to_owned(),
        url: "https://google.com".to_owned(),
      };

    pub static ref TEST_SHORTCUT: Shortcut = Shortcut { 
        id: Some(3),
        created: Some(111), 
        updated: Some(222), 
        keyword: "test".to_owned(),
        url: "https://test.co.uk".to_owned(),
      };
  }
  
  async fn bulk_insert(pool: Pool<Sqlite>) -> Pool<Sqlite> {
    // Create temporary table within memory
    pool.execute("CREATE TABLE IF NOT EXISTS shortcuts (
        id        INTEGER UNIQUE,
        created   INTEGER NOT NULL,
        updated   INTEGER NOT NULL,
        keyword   TEXT  NOT NULL  UNIQUE,
        url       TEXT  NOT NULL,
        PRIMARY KEY(id ASC)
      );").await.unwrap();

    // Batch insert test data
    let query = "
        INSERT INTO shortcuts (created, updated, keyword, url) VALUES
        (1, 2, 'google', 'https://google.co.uk'), 
        (11, 22, 'g', 'https://google.com'), 
        (111, 222, 'test', 'https://test.co.uk'), 
        (1111, 2222, 'gle', 'https://greatlightexchange.co.uk')
    ";
    pool.execute(query).await.unwrap();

    // Verify count
    pool
  }

  async fn mock_pool() -> Pool<Sqlite> {
    bulk_insert(SqlitePool::connect("sqlite::memory:").await.unwrap()).await
  }

  async fn setup() -> ShortcutRepository {
    ShortcutRepository::new(mock_pool().await)
  }

  mod get_tests {
    use crate::{error::ShortcutError, repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait}};

    use super::{setup, GOOGLE_SHORTCUT};

    #[tokio::test]
    async fn gets_exact_match_when_exists() {
      let input = "google".to_string();
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.get(&input).await.unwrap();
  
      assert_eq!(result, GOOGLE_SHORTCUT.to_owned());
    }
  
    #[tokio::test]
    async fn returns_exception_when_none_exists() {
      let input = "bing".to_string();
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.get(&input).await;
  
      assert!(matches!(result, Err(ShortcutError::NotFound)));
    }
    
    #[tokio::test]
    async fn returns_exception_when_empty_input() {
      let input = "".to_string();
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.get(&input).await;
  
      assert!(matches!(result, Err(ShortcutError::NotFound)));
    }
  }

  mod create_tests {
    use crate::{error::ShortcutError, repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait}};

    use super::{setup, GOOGLE_SHORTCUT, BING_SHORTCUT};

    #[tokio::test]
    async fn gets_true_when_unique_entry() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.create(&BING_SHORTCUT).await.unwrap();
  
      assert!(result);
    }
  
    #[tokio::test]
    async fn returns_exception_when_already_exists() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.create(&GOOGLE_SHORTCUT).await;
  
      assert!(matches!(result, Err(ShortcutError::UniqueConstraintError)));
    }
  }

  mod update_test {
    use crate::repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait};

    use super::{setup, GOOGLE_COM_SHORTCUT, BING_SHORTCUT};

    #[tokio::test]
    async fn returns_true_when_exists() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.update(&GOOGLE_COM_SHORTCUT).await.unwrap();
      
      assert!(result);
    }
  
    #[tokio::test]
    async fn returns_false_when_not_exists() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.update(&BING_SHORTCUT).await.unwrap();
  
      assert!(!result);
    }
  }

  mod delete_test {
    use crate::repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait};

    use super::{setup, GOOGLE_COM_SHORTCUT, BING_SHORTCUT};

    #[tokio::test]
    async fn returns_true_when_exists() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.delete(GOOGLE_COM_SHORTCUT.keyword.as_str()).await.unwrap();
      
      assert!(result);
    }
  
    #[tokio::test]
    async fn returns_false_when_not_exists() {
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.delete(BING_SHORTCUT.keyword.as_str()).await.unwrap();
  
      assert!(!result);
    }
  }

  mod fuzzy_search_test {
    use crate::{error::ShortcutError, repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait}};

    use super::{setup, G_SHORTCUT, GLE_SHORTCUT, GOOGLE_SHORTCUT, TEST_SHORTCUT};

    #[tokio::test]
    async fn returns_one_entry_when_exists() {
      let input = "test";
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.fuzzy_search(input).await.unwrap();
 
      let expected = vec![TEST_SHORTCUT.to_owned()];

      assert_eq!(result, expected);
    }
  
    #[tokio::test]
    async fn returns_ordered_multiple_entry_when_exists() {
      let input = "google";
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.fuzzy_search(input).await.unwrap();
 
      let expected = vec![
        GOOGLE_SHORTCUT.to_owned(), 
        GLE_SHORTCUT.to_owned(), 
        G_SHORTCUT.to_owned()
      ];

      assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn returns_error_when_no_matches_exists() {
      let input = "bing";
      let shortcut_repository: ShortcutRepository = setup().await;
      let result = shortcut_repository.fuzzy_search(input).await;

      assert!(matches!(result, Err(ShortcutError::NoMatches)));
    }
  }
}

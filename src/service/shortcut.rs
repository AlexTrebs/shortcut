use axum::response::Result;
use sqlx::Error;

use crate::{
  error::ShortcutError,
  models::shortcut::{PostRequest, Shortcut},
  repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait},
};

#[derive(Clone)]
pub struct ShortcutService {
  repository: ShortcutRepository,
}

pub trait ShortcutServiceTrait {
  fn new(repository: ShortcutRepository) -> Self;
  async fn find_similar(&self, req: &String) -> Result<Vec<Shortcut>, ShortcutError>;
  async fn create(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError>;
  async fn update(&self, req: &PostRequest) -> Result<bool, ShortcutError>;
  async fn get(&self, req: &String) -> Result<Shortcut, ShortcutError>;
}

impl ShortcutServiceTrait for ShortcutService {
  fn new(repository: ShortcutRepository) -> Self {
    Self {
      repository
    }
  }

  async fn find_similar(&self, req: &String) -> Result<Vec<Shortcut>, ShortcutError> {
    let result: Result<Vec<Shortcut>, ShortcutError> = self.repository.fuzzy_search(req).await;
    
    match result {
      Ok(todos) => Ok(todos),
      Err(_err) => Err(ShortcutError::FailedToGet),
    }
  }

  async fn create(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError> {
    let result: Result<bool, Error> = self.repository.create(&shortcut).await;
    
    match result {
      Ok(success) => Ok(success),
      Err(Error::Database(db_err)) if db_err.message().contains("UNIQUE constraint failed") => Err(ShortcutError::UniqueConstraintError),
      Err(_) => Err(ShortcutError::FailedToCreate),
    }
  }

  async fn update(&self, req: &PostRequest) -> Result<bool, ShortcutError> {
    let shortcut: Shortcut = Shortcut::from_request(req);
    let result: Result<bool, ShortcutError> = self.repository.update(&shortcut).await;
    
    match result {
      Ok(success) => Ok(success),
      Err(_err) => Err(ShortcutError::FailedToUpdate),
    }
  }

  async fn get(&self, keyword: &String) -> Result<Shortcut, ShortcutError> {
    let result: Result<Shortcut, ShortcutError> = self.repository.get(&keyword).await;
    
    match result {
      Ok(success) => Ok(success),
      Err(_err) => Err(ShortcutError::FailedToGet),
    }
  }
}
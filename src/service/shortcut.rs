use axum::response::Result;

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
  async fn create_or_update(&self, req: &PostRequest) -> Result<String, ShortcutError>;
  async fn update(&self, req: &Shortcut) -> Result<bool, ShortcutError>;
  // async fn get(&self, req: SearchRequest) -> Result<Shortcut, ShortcutError>;
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

  async fn create_or_update(&self, req: &PostRequest) -> Result<String, ShortcutError> {
    let shortcut: Shortcut = Shortcut::from_request(req);
    let result: Result<bool, ShortcutError> = self.repository.create(&shortcut).await;
    
    match result {
      Ok(_) => Ok("created".to_string()),
      Err(_err) => {
        match self.update(&shortcut).await {
          Ok(_) => Ok("updated".to_string()),
          Err(_err) => Err(ShortcutError::FailedToCreateOrUpdate),
        }
      },
    }
  }

  async fn update(&self, shortcut: &Shortcut) -> Result<bool, ShortcutError> {
    let result: Result<bool, ShortcutError> = self.repository.update(&shortcut).await;
    
    match result {
      Ok(todos) => Ok(todos),
      Err(_err) => Err(ShortcutError::FailedToUpdate),
    }
  }
}
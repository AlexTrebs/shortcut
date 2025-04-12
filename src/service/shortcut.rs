use crate::{
  error::ShortcutError, 
  macros::renderable::Renderable,
  models::shortcut::{PostRequest, Shortcut}, 
  repository::shortcut::ShortcutRepositoryTrait, 
  templates::components::{CheckUpdateTemplate, CreateNewTemplate, ErrorTemplate, InfoTemplate, SearchResultsTemplate, SuccessTemplate}, 
  TERA
};

use axum::response::{Html, Redirect, Result};
use tera::Tera;

#[derive(Clone)]
pub struct ShortcutService<R: ShortcutRepositoryTrait + Send + Sync> {
  repository: R,
}

/// Defines the Shortcut Service Trait with required operations.
impl <R: ShortcutRepositoryTrait + Send + Sync> ShortcutService<R> {
  /// Creates a new instance of `ShortcutService`.
  pub fn new(repository: R) -> Self {
    Self {
      repository
    }
  }
  
  /// Searches for shortcuts that are similar to the given keyword using fuzzy matching.
  ///
  /// # Parameters
  /// - `keyword`: The search query string.
  ///
  /// # Returns
  /// - `Html<String>` containing matching results or suggestions.
  pub async fn find_similar(&self, keyword: &str) -> Html<String> {
    let result: Result<Vec<Shortcut>, ShortcutError> = self.repository.fuzzy_search(keyword).await;
    let tera:Tera = TERA.read().unwrap().clone();
  
    match result {
      Ok(shortcuts) => {
        let context: SearchResultsTemplate = SearchResultsTemplate { shortcuts, query: keyword.to_owned() };
        
        context.get_html(tera)
      },
      Err(ShortcutError::NoMatches) => {
        let context: CreateNewTemplate = CreateNewTemplate { keyword: keyword.to_owned() };

        context.get_html(tera)
      }
      Err(err) => {
        let context: ErrorTemplate = ErrorTemplate { error:err.to_string(), successful: false };
        
        context.get_html(tera)
      }
    }
  }

  /// Creates a new shortcut and handles duplicate entries.
  ///
  /// # Parameters
  /// - `shortcut`: The data to create the new shortcut.
  ///
  /// # Returns
  /// - `Html<String>` indicating success or failure.
  pub async fn create(&self, params: &PostRequest) -> Html<String> {
    let new_shortcut: Shortcut = Shortcut::from_request(params);
    let result: Result<bool, ShortcutError> = self.repository.create(&new_shortcut).await;
    let tera: Tera = TERA.read().unwrap().clone();
  
    match result {
      Ok(_) => {
        let context: SuccessTemplate = SuccessTemplate { message: "Successfully created shortcut!".into(), successful: true };
  
        context.get_html(tera)
      },
      Err(ShortcutError::UniqueConstraintError) => match self.repository.get(&new_shortcut.keyword).await {
        Ok(shortcut_to_update) => {
          if shortcut_to_update.url == new_shortcut.url {
            let context = InfoTemplate { message: "Shortcut already added.".into(), successful: true };
            
            context.get_html(tera)
          } else {
            let context: CheckUpdateTemplate = CheckUpdateTemplate { shortcut: shortcut_to_update, new_url: new_shortcut.url, successful: false };
            
            context.get_html(tera)
          }
        },
        Err(err) => {
          let context: ErrorTemplate = ErrorTemplate { error:err.to_string(), successful: false };
          
          context.get_html(tera)
        }
      },
      Err(err) => {
        let context: ErrorTemplate = ErrorTemplate { error:err.to_string(), successful: false };
        
        context.get_html(tera)
      }
    }
  }

  /// Updates an existing shortcut.
  ///
  /// # Parameters
  /// - `req`: The updated shortcut request.
  ///
  /// # Returns
  /// - `Html<String>` indicating success or failure.
  pub async fn update(&self, req: &PostRequest) -> Html<String> {
    let shortcut: Shortcut = Shortcut::from_request(req);
    let result: Result<bool, ShortcutError> = self.repository.update(&shortcut).await;
    let tera:Tera = TERA.read().unwrap().clone();
  
    match result {
      Ok(_) => {
        let message: String = "Successfully updated shortcut!".to_string();
        let context: SuccessTemplate = SuccessTemplate { message, successful: true };
        
        context.get_html(tera)
      },
      Err(err) => {
        let context: ErrorTemplate = ErrorTemplate { error:err.to_string(), successful: false };
        
        context.get_html(tera)
      }
    }
  }
  /// Retrieves a shortcut by keyword and returns a redirect to its URL.
  ///
  /// # Parameters
  /// - `keyword`: The shortcut's keyword.
  ///
  /// # Returns
  /// - `Redirect` that redirects the user to the corresponding URL.
  pub async fn get(&self, keyword: &str) -> Redirect {
    return match self.repository.get(keyword).await {
      Ok(shortcut) =>Redirect::permanent(&shortcut.url),
      Err(ShortcutError::NotFound) => Redirect::permanent(&format!("/search?keyword={}", keyword)),
      Err(_) => Redirect::permanent(&(std::env::var("UI_URL").unwrap_or(String::from("http://localhost:3000")))),
    };
  }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[cfg(test)]
mod shortcut_repository_tests {
  use lazy_static::lazy_static;

  use crate::models::shortcut::Shortcut;

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

  mod get_tests {
    use axum::response::Redirect;

    use crate::{
      error::ShortcutError, 
      repository::shortcut::MockShortcutRepositoryTrait, 
      service::shortcut::ShortcutService
    };

    use super::GOOGLE_SHORTCUT;

    #[tokio::test]
    async fn get_redirect_to_shortcut_when_exists() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_get().returning(|_| Ok(GOOGLE_SHORTCUT.to_owned()));

      let shortcut_service = ShortcutService::new(mock);

      let input = "google".to_string();
      let result = shortcut_service.get(&input).await;
      let _expected_redirect = Redirect::permanent(&GOOGLE_SHORTCUT.url);

      assert!(matches!(result, _expected_redirect))
    }

    #[tokio::test]
    async fn get_redirect_to_search_when_not_exists() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_get().returning(|_| Err(ShortcutError::NotFound));

      let shortcut_service = ShortcutService::new(mock);

      let input = "google".to_string();
      let result = shortcut_service.get(&input).await;
      let _expected_redirect = Redirect::permanent(&format!("/search?keyword={}", input));

      assert!(matches!(result, _expected_redirect))
    }
  }

  mod update_tests {
    use tera::Tera;

    use crate::{
      error::ShortcutError, 
      macros::renderable::Renderable, 
      models::shortcut::PostRequest, 
      repository::shortcut::MockShortcutRepositoryTrait, 
      service::shortcut::ShortcutService,
      templates::components::{ErrorTemplate, SuccessTemplate}, 
      TERA
    };

    #[tokio::test]
    async fn return_success_html_when_updated() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_update().returning(|_| Ok(true));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.update(&input).await;

      let message: String = "Successfully updated shortcut!".to_string();
      let tera:Tera = TERA.read().unwrap().clone();
      let context: SuccessTemplate = SuccessTemplate { message, successful: true };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }

    #[tokio::test]
    async fn return_error_html_when_failed_to_update() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_update().returning(|_| Err(ShortcutError::FailedToUpdate));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.update(&input).await;
      
      let tera:Tera = TERA.read().unwrap().clone();
      let context: ErrorTemplate = ErrorTemplate { error: ShortcutError::FailedToUpdate.to_string(), successful: false };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
  }

  mod create_tests {
    use tera::Tera;

    use crate::{
      error::ShortcutError, 
      macros::renderable::Renderable, 
      models::shortcut::PostRequest, 
      repository::shortcut::MockShortcutRepositoryTrait, 
      service::shortcut::{shortcut_repository_tests::{GOOGLE_COM_SHORTCUT, GOOGLE_SHORTCUT}, ShortcutService}, 
      templates::components::{CheckUpdateTemplate, ErrorTemplate, InfoTemplate, SuccessTemplate}, 
      TERA
    };

    #[tokio::test]
    async fn return_success_html_when_updated() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_create().returning(|_| Ok(true));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.create(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();
      let context: SuccessTemplate = SuccessTemplate { message: "Successfully created shortcut!".into(), successful: true };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }

    #[tokio::test]
    async fn return_info_html_when_shortcut_already_created() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_create().returning(|_| Err(ShortcutError::UniqueConstraintError));
      mock.expect_get().returning(|_| Ok(GOOGLE_SHORTCUT.to_owned()));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.create(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();
      let context = InfoTemplate { message: "Shortcut already added.".into(), successful: true };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }

    #[tokio::test]
    async fn return_error_html_when_shortcut_already_created() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_create().returning(|_| Err(ShortcutError::UniqueConstraintError));
      mock.expect_get().returning(|_| Ok(GOOGLE_COM_SHORTCUT.to_owned()));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.create(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();            
      let context: CheckUpdateTemplate = CheckUpdateTemplate { shortcut: GOOGLE_COM_SHORTCUT.to_owned(), new_url: input.url, successful: false };
            
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
    
    #[tokio::test]
    async fn return_error_html_when_shortcut_already_created_and_get_error() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_create().returning(|_| Err(ShortcutError::UniqueConstraintError));
      mock.expect_get().returning(|_| Err(ShortcutError::FailedToGet));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.create(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();            
      let context: ErrorTemplate = ErrorTemplate { error: ShortcutError::FailedToGet.to_string(), successful: false };
            
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }

    #[tokio::test]
    async fn return_error_html_when_failed_() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_create().returning(|_| Err(ShortcutError::FailedToCreate));

      let shortcut_service = ShortcutService::new(mock);

      let input = PostRequest { keyword: "google".to_owned(), url: "https://google.co.uk".to_owned() };
      let result = shortcut_service.create(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();            
      let context: ErrorTemplate = ErrorTemplate { error: ShortcutError::FailedToCreate.to_string(), successful: false };
            
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
  }

  mod find_similar_tests {
    use tera::Tera;

    use crate::{
      error::ShortcutError, 
      macros::renderable::Renderable, 
      models::shortcut::Shortcut, 
      repository::shortcut::MockShortcutRepositoryTrait, 
      service::shortcut::{shortcut_repository_tests::{GLE_SHORTCUT, GOOGLE_SHORTCUT, G_SHORTCUT}, ShortcutService}, 
      templates::components::{CreateNewTemplate, ErrorTemplate, SearchResultsTemplate}, 
      TERA
    };

    #[tokio::test]
    async fn return_success_html_when_matches_found() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_fuzzy_search().returning(|_| Ok(vec!(GOOGLE_SHORTCUT.to_owned(), GLE_SHORTCUT.to_owned(), G_SHORTCUT.to_owned())));

      let shortcut_service = ShortcutService::new(mock);

      let input = "google".to_owned();
      let result = shortcut_service.find_similar(&input).await;
      let shortcuts: Vec<Shortcut> = vec!(GOOGLE_SHORTCUT.to_owned(), GLE_SHORTCUT.to_owned(), G_SHORTCUT.to_owned());

      let tera:Tera = TERA.read().unwrap().clone();
      let context: SearchResultsTemplate = SearchResultsTemplate { shortcuts: shortcuts.to_owned(), query: input };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
    
    #[tokio::test]
    async fn return_create_new_html_when_no_matches_found() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_fuzzy_search().returning(|_| Err(ShortcutError::NoMatches));

      let shortcut_service = ShortcutService::new(mock);

      let input = "google".to_owned();
      let result = shortcut_service.find_similar(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();
      let context: CreateNewTemplate = CreateNewTemplate { keyword: input };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
        
    #[tokio::test]
    async fn return_error_html_when_error_returned() {
      let mut mock: MockShortcutRepositoryTrait = MockShortcutRepositoryTrait::default(); 
      mock.expect_fuzzy_search().returning(|_| Err(ShortcutError::FailedToSearch));

      let shortcut_service = ShortcutService::new(mock);

      let input = "google".to_owned();
      let result = shortcut_service.find_similar(&input).await;

      let tera:Tera = TERA.read().unwrap().clone();
      let context: ErrorTemplate = ErrorTemplate { error:ShortcutError:: FailedToSearch.to_string(), successful: false };
      
      let _expected_result = context.get_html(tera);

      assert_eq!(result.0, _expected_result.0)
    }
  }
}

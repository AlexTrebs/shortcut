use crate::{macros::renderable::Renderable, models::shortcut::Shortcut};

use axum::response::Html;
use serde::Serialize;
use tera::Tera;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/common/error.html")]
pub struct ErrorTemplate {
  pub error: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/common/info.html")]
pub struct InfoTemplate {
  pub message: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/common/success.html")]
pub struct SuccessTemplate {
  pub message: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/search/search_results.html")]
pub struct SearchResultsTemplate {
  pub shortcuts: Vec<Shortcut>,
  pub query: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/create/check_update.html")]
pub struct CheckUpdateTemplate {
  pub shortcut: Shortcut,
  pub new_url: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/search/create_new.html")]
pub struct CreateNewTemplate {
  pub keyword: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/common/empty.html")]
pub struct EmptyTemplate {}


impl_renderable!(
  SearchResultsTemplate, 
  SuccessTemplate, 
  CheckUpdateTemplate, 
  ErrorTemplate, 
  InfoTemplate, 
  CreateNewTemplate,
  EmptyTemplate
);

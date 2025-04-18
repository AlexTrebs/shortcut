use crate::{macros::renderable::Renderable, models::shortcut::Shortcut};

use axum::response::Html;
use serde::Serialize;
use tera::Tera;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/alert/error.html")]
pub struct ErrorAlert {
  pub error: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/alert/info.html")]
pub struct InfoAlert {
  pub message: String,
  pub successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/alert/success.html")]
pub struct SuccessAlert {
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

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/dialog/success.html")]
pub struct SuccessDialog {
  pub title: String,
  pub message: String,
  pub keyword: String,
  pub action: String,
  pub status: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/dialog/error.html")]
pub struct ErrorDialog {
  pub title: String,
  pub message: String,
  pub keyword: String,
  pub action: String,
  pub status: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/dialog/info.html")]
pub struct InfoDialog {
  pub title: String,
  pub message: String,
  pub keyword: String,
  pub action: String,
  pub status: String,
}

impl_renderable!(
  SearchResultsTemplate, 
  SuccessAlert, 
  CheckUpdateTemplate, 
  ErrorAlert, 
  InfoAlert, 
  CreateNewTemplate,
  EmptyTemplate,
  SuccessDialog,
  ErrorDialog,
  InfoDialog
);

use crate::{
  error::ShortcutError, 
  models::shortcut::PostRequest, 
  service::shortcut::ShortcutServiceTrait, 
  state::AppState, 
  TERA,
};

use axum::{
  response::Html, Extension, Form
};

use serde::Serialize;
use tera_hot_reload::TeraTemplate;
use tracing::debug;

pub async fn update_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<PostRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);
  let result: Result<bool, ShortcutError> = app.shortcut_service.update(&params).await;

  return match result {
    Ok(_) => {
      let message: String = format!("Successfully updated shortcut!").to_string();
      let context = UpdateSuccessTemplate { message, query: params.keyword, successful: true };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    },
    Err(err) => {
      let context = UpdateErrorTemplate { error:err.to_string(), query: params.keyword, successful: false };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    }
  };
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/success.html")]
pub struct UpdateSuccessTemplate {
  message: String,
  query: String,
  successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/error.html")]
pub struct UpdateErrorTemplate {
  error: String,
  query: String,
  successful: bool,
}
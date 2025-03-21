use crate::{
  error::ShortcutError, 
  models::shortcut::PostRequest, 
  service::shortcut::ShortcutServiceTrait, 
  state::AppState, 
  TERA,
};

use axum::{
  extract::Query, response::Html, Extension
};

use serde::Serialize;
use tera_hot_reload::TeraTemplate;
use tracing::debug;

pub async fn post_shortcut(
  Extension(app): Extension<AppState>,
  Query(params): Query<PostRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);
  let result: Result<String, ShortcutError> = app.shortcut_service.create_or_update(&params).await;

  return match result {
    Ok(status) => {
      let message: String = format!("Successfully {} shortcut!", status).to_string();
      let context = PostSuccessTemplate { message, query: params.keyword };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    },
    Err(err) => {
      let context = PostErrorTemplate { error:err.to_string(), query: params.keyword };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    }
  };
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "post/post_success.html")]
pub struct PostSuccessTemplate {
  message: String,
  query: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "post/post_failed.html")]
pub struct PostFailedTemplate {
  message: String,
  query: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/error.html")]
pub struct PostErrorTemplate {
  error: String,
  query: String,
}
use crate::{
  error::ShortcutError, 
  models::shortcut::{PostRequest, Shortcut}, 
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

pub async fn post_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<PostRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);
  let shortcut: Shortcut = Shortcut::from_request(&params);
  let result: Result<bool, ShortcutError> = app.shortcut_service.create(&shortcut).await;

  return match result {
    Ok(_) => {
      let message: String = "Successfully updated shortcut!".to_string();
      let context = PostSuccessTemplate { message, query: params.keyword, successful: true };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    },
    Err(ShortcutError::UniqueConstraintError) => {
      let shortcut_result = app.shortcut_service.get(&shortcut.keyword).await;
      match shortcut_result {
          Ok(shortcut_to_update) => {
            let context = CheckUpdateTemplate { shortcut: shortcut_to_update, query: params.keyword, successful: true };
            let rendered = context.render(TERA.read().unwrap().clone());
      
            Ok(Html(rendered))
          },
          Err(err) => {
            let context = PostErrorTemplate { error:err.to_string(), query: params.keyword, successful: false };
            let rendered = context.render(TERA.read().unwrap().clone());
      
            Ok(Html(rendered))
          }
      }

    },
    Err(err) => {
      let context = PostErrorTemplate { error:err.to_string(), query: params.keyword, successful: false };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    }
  };
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/success.html")]
pub struct PostSuccessTemplate {
  message: String,
  query: String,
  successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/check_update.html")]
pub struct CheckUpdateTemplate {
  shortcut: Shortcut,
  query: String,
  successful: bool,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/check_update.html")]
pub struct PostErrorTemplate {
  error: String,
  query: String,
  successful: bool,
}
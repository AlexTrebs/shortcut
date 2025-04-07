use std::env;

use crate::{
  error::ShortcutError, 
  models::shortcut::{SearchRequest, Shortcut}, 
  service::shortcut::ShortcutServiceTrait, 
  state::AppState, 
};

use axum::{
  response::Redirect, Extension, Form
};

use tracing::{debug, error};

pub async fn redirect_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<SearchRequest>,
) -> Result<Redirect, ()> {
  debug!("{:?}", params.keyword);
  let result: Result<Shortcut, ShortcutError> = app.shortcut_service.get(&params.keyword).await;

  return match result {
    Ok(shortcut) => Ok(Redirect::temporary(&shortcut.url)),
    Err(ShortcutError::NotFound) => {
      error!("FOUND");
      Ok(Redirect::temporary(&(env::var("UI_URL").expect("DATABASE_FILENAME not set") + "/search?keyword=" + &params.keyword)))
    },
    Err(_) => Ok(Redirect::temporary(&(env::var("UI_URL").expect("DATABASE_FILENAME not set")))),
  };
}

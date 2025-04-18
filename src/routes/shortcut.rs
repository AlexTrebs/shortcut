use crate::{
  macros::renderable::Renderable,
  models::shortcut::{PostRequest, KeywordRequest}, 
  state::AppState, 
  templates::components::EmptyTemplate, 
  TERA, 
};

use axum::{
  extract::Query, response::{Html, Redirect}, Extension, Form
};

use tera::Tera;
use tracing::debug;

/// Used for when update is retrieved, but `params` are invalid to reduce load on db.
pub async fn get_empty_html() -> Html<String> {
  let tera:Tera = TERA.read().unwrap().clone();
  let context: EmptyTemplate = EmptyTemplate {};

  context.get_html(tera)
}

/// This is the fuction for the `/api/search` endpoint. 
/// 
/// It will call the `Shortcut_Service` function `find_similar` function and return the result. 
/// 
/// If the keyword inputted is empty, it will instead return an empty `Html` `String`.
pub async fn search_shortcuts(
  Extension(app): Extension<AppState>,
  Query(params): Query<KeywordRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);

  if params.keyword.is_empty() {
    return Ok(get_empty_html().await);
  }

  Ok(app.shortcut_service.find_similar(&params.keyword.to_lowercase()).await)
}

/// This is the function for the `/api/post` endpoint.
/// 
/// It will call the `Shortcut_Service` function `create` and return the result.
pub async fn post_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<PostRequest>,
) -> Result<Html<String>, ()> {
  debug!("{}: {}", params.keyword, params.url);

  Ok(app.shortcut_service.create(&params).await)
}

/// This is the function for the `/api/get` endpoint.
/// 
/// It will call the `Shortcut_Service` function `get` and return the result.
pub async fn redirect_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<KeywordRequest>,
) -> Result<Redirect, ()> {
  debug!("{:?}", params.keyword);

  Ok(app.shortcut_service.get(&params.keyword.to_lowercase()).await)
}

/// This is the function for the `/api/update` endpoint.
/// 
/// It will call the `Shortcut_Service` function `update` and return the result.
pub async fn update_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<PostRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);

  Ok(app.shortcut_service.update(&params).await)
}

/// This is the function for the `/api/delete` endpoint.
/// 
/// It will call the `Shortcut_Service` function `delete` and return the result.
pub async fn delete_shortcut(
  Extension(app): Extension<AppState>,
  Form(params): Form<KeywordRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);

  Ok(app.shortcut_service.delete(params.keyword.as_str()).await)
}

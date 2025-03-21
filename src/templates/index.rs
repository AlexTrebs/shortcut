use crate::{TERA, state::AppState};

use axum::{Extension, response::{Html, IntoResponse}};
use serde::Serialize;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub async fn get_index(Extension(app): Extension<AppState>) -> impl IntoResponse {
  let context = IndexTemplate {};

  Html(context.render(TERA.read().unwrap().clone()))
}
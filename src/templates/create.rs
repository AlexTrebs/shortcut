use crate::TERA;

use axum::response::{Html, IntoResponse};
use serde::Serialize;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "createPage.html")]
pub struct CreatePageTemplate {
  current_page: String,
}

pub async fn get_create() -> impl IntoResponse {
  let context = CreatePageTemplate { current_page: "String".to_string() };

  Html(context.render(TERA.read().unwrap().clone()))
}
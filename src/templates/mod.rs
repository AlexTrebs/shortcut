use crate::{TERA, macros::renderable::Renderable};

use axum::{http::StatusCode, response::{IntoResponse, Redirect}, routing::get, Router};
use pages::{CreatePageTemplate, SearchPageTemplate};

pub mod components;
pub mod pages;

async fn redirect_to_search() -> Redirect {
  Redirect::to("/search")
}

async fn health_check() -> impl IntoResponse {
  // Return HTTP 200 OK to signify the application is healthy
  StatusCode::OK
}

pub fn create_ui_routes() -> Router {
  Router::new()
    .route("/", get(redirect_to_search))
    .route("/healthcheck", get(health_check))
    .route("/search", get(SearchPageTemplate{}.get_html(TERA.read().unwrap().clone())))
    .route("/create", get(CreatePageTemplate{}.get_html(TERA.read().unwrap().clone())))
}

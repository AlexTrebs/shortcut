use crate::{TERA, macros::renderable::Renderable};

use axum::{response::Redirect, routing::get, Router};
use pages::{CreatePageTemplate, SearchPageTemplate};

pub mod components;
pub mod pages;

async fn redirect_to_search() -> Redirect {
  Redirect::to("/search")
}

pub fn create_ui_routes() -> Router {
  Router::new()
  .route("/", get(redirect_to_search))
  .route("/search", get(SearchPageTemplate{}.get_html(TERA.read().unwrap().clone())))
  .route("/create", get(CreatePageTemplate{}.get_html(TERA.read().unwrap().clone())))
}

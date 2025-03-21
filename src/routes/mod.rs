use axum::{
  middleware::from_fn,
  routing::get,
  Router,
};

use self::{
  middleware::get_htmx_header,
  post::post_shortcut,
  search::get_shortcuts,
};

pub mod middleware;
pub mod post;
pub mod search;

pub fn create_api_routes() -> Router {
  Router::new()
    .route("/search", get(get_shortcuts))
    .route("/add", get(post_shortcut))
    .layer(from_fn(get_htmx_header))
}
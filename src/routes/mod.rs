use axum::{
  middleware::from_fn,
  routing::{get, post},
  Router,
};

use self::{
  middleware::get_htmx_header,
  post::post_shortcut,
  search::get_shortcuts,
  update::update_shortcut,
};

pub mod middleware;
pub mod post;
pub mod search;
pub mod update;

pub fn create_api_routes() -> Router {
  Router::new()
    .route("/search", get(get_shortcuts))
    .route("/post", post(post_shortcut))
    .route("/update", post(update_shortcut))
    .layer(from_fn(get_htmx_header))
}
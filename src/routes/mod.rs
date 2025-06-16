use axum::{
  middleware::from_fn,
  routing::{delete, get, post},
  Router,
};

use self::{
  middleware::get_htmx_header,
  shortcut::post_shortcut,
  shortcut::redirect_shortcut,
  shortcut::search_shortcut,
  shortcut::update_shortcut,
  shortcut::delete_shortcut,
};

pub mod middleware;
pub mod shortcut;

pub fn create_api_routes() -> Router {
  Router::new()
    .route("/search", get(search_shortcut))
    .route("/post", post(post_shortcut))
    .route("/get", get(redirect_shortcut))
    .route("/update", post(update_shortcut))
    .route("/delete", delete(delete_shortcut))
    .layer(from_fn(get_htmx_header))
}

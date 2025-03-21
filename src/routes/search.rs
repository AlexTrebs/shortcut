use crate::{
  TERA,
  error::ShortcutError,
  models::shortcut::{Shortcut, SearchRequest},
  service::shortcut::ShortcutServiceTrait,
  state::AppState,
};

use axum::{
  extract::Query, response::Html, Extension
};

use serde::Serialize;
use tera_hot_reload::TeraTemplate;
use tracing::debug;

pub async fn get_shortcuts(
  Extension(app): Extension<AppState>,
  Query(params): Query<SearchRequest>,
) -> Result<Html<String>, ()> {
  debug!("{:?}", params.keyword);
  let result: Result<Vec<Shortcut>, ShortcutError> = app.shortcut_service.find_similar(&params.keyword).await;

  return match result {
    Ok(shortcuts) => {
      let context = SearchTemplate { shortcuts, query: params.keyword };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    },
    Err(err) => {
      let context = SearchErrorTemplate { error:err.to_string(), query: params.keyword };
      let rendered = context.render(TERA.read().unwrap().clone());

      Ok(Html(rendered))
    }
  };
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "search/search_results.html")]
pub struct SearchTemplate {
  shortcuts: Vec<Shortcut>,
  query: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/error.html")]
pub struct SearchErrorTemplate {
  error: String,
  query: String,
}
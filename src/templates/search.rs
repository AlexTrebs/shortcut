use crate::models::shortcut::Shortcut;
use serde::Serialize;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "search/search_results.html")]
pub struct SearchTemplate {
  shortcuts: Vec<Shortcut>,
  query: String,
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "components/error.html")]
pub struct SearchErrorrTemplate {
  error: String,
  query: String,
}
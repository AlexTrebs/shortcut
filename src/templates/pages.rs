use crate::macros::renderable::Renderable;

use axum::response::Html;
use serde::Serialize;
use tera::Tera;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "pages/createPage.html")]
pub struct CreatePageTemplate {}

#[derive(TeraTemplate, Serialize)]
#[template(path = "pages/searchPage.html")]
pub struct SearchPageTemplate {}

impl_renderable!(SearchPageTemplate, CreatePageTemplate);

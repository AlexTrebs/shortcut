use crate::utils::system_util::get_timestamp;

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Shortcut {
  pub id: i64,
  pub created: i64,
  pub updated: i64,
  pub keyword: String,
  pub url: String,
}

impl Shortcut { 
  pub fn new(keyword: String, url: String) -> Self {
    let timestamp = get_timestamp();
    Self {
        id: 0,
        created: timestamp,
        updated: timestamp,
        keyword,
        url,
    }
  }

  pub fn from_request(request: &PostRequest) -> Self {
    let timestamp = get_timestamp();
    Self {
        id: 0,
        created: timestamp,
        updated: timestamp,
        keyword: request.keyword.clone(),
        url: request.url.clone(),
    }
  }
}

#[derive(Deserialize)]
pub struct SearchRequest {
  pub keyword: String,
}

#[derive(Deserialize)]
pub struct PostRequest {
  pub keyword: String,
  pub url: String,
}

#[derive(Serialize)]
pub struct ListShortcutsResponse {
  pub shortcuts: Vec<Shortcut>,
  pub success: bool,
  pub error: String,
  pub status_code: u16,
}

impl ListShortcutsResponse {
  pub fn success(shortcuts: Vec<Shortcut>) -> Self {
    Self {
      status_code: StatusCode::OK.as_u16(),
      success: true,
      error: "".to_string(),
      shortcuts,
    }
  }

  pub fn error(status_code: u16, error: String) -> Self {
    Self {
      status_code,
      success: false,
      error,
      shortcuts: vec![],
    }
  }
}
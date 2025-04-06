use crate::utils::system_util::get_timestamp;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Shortcut {
  pub id: Option<i64>,
  pub created: Option<i64>,
  pub updated: Option<i64>,
  pub keyword: String,
  pub url: String,
}

impl Shortcut { 
  pub fn new(keyword: String, url: String) -> Self {
    let timestamp = get_timestamp();
    Self {
        id: None,
        created: Some(timestamp),
        updated: Some(timestamp),
        keyword,
        url,
    }
  }

  pub fn from_request(request: &PostRequest) -> Self {
    let timestamp = get_timestamp();
    Self {
        id: None,
        created: Some(timestamp),
        updated: Some(timestamp),
        keyword: request.keyword.to_lowercase().clone(),
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
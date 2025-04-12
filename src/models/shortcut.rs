use crate::utils::system_util::get_timestamp;

use serde::{Deserialize, Serialize};

/// A model used for the datamodel of the shortcuts saved.
/// 
/// There are 5 fields of this structure:
/// - `id` which will be generated on creation in the db,
/// - `create` which will be generated on first creation of the `Shortcut` object,
/// - `updated` which will be generated on first creation and further updates of the `Shortcut` object,
/// - `keyword` which is unique,
/// - `url` which is unique.
/// 
/// The `keyword` and `url` is required by both construcors.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Shortcut {
  pub id: Option<i64>,
  pub created: Option<i64>,
  pub updated: Option<i64>,
  pub keyword: String,
  pub url: String,
}

impl Shortcut { 
  /// A constructor which takes the default fields required to create `Shortcut`.
  #[allow(dead_code)]
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

  /// A constructor which takes the `PostRequest` object and creates a `Shortcut` from it.
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

/// Required to cast request within search endpoints to object.
#[derive(Deserialize)]
pub struct SearchRequest {
  pub keyword: String,
}

/// Required to cast request within post endpoints to object.
#[derive(Deserialize)]
pub struct PostRequest {
  pub keyword: String,
  pub url: String,
}
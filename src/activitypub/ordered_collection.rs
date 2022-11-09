use serde::{Deserialize, Serialize};

use crate::helpers::handlers::div_up;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedCollectionMetadata {
  #[serde(rename = "@context")]
  pub context: &'static str,
  pub first: String,
  pub last: String,
  pub id: String,
  #[serde(rename = "totalItems")]
  pub total_items: i64,
  #[serde(rename = "type")]
  pub object_type: &'static str,
}

impl OrderedCollectionMetadata {
  pub fn build(subject_url: &str, total_items: i64, page_size: i64) -> OrderedCollectionMetadata {
    OrderedCollectionMetadata {
      context: "https://www.w3.org/ns/activitystreams",
      first: format!("{}?page=0&limit={}", subject_url, page_size),
      last: format!(
        "{}?page={}&limit={}",
        subject_url,
        div_up(total_items, page_size),
        page_size
      ),
      id: subject_url.to_string(),
      total_items: total_items,
      object_type: "OrderedCollection",
    }
  }
}

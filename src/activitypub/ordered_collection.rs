use serde::{Deserialize, Serialize};
use std::vec;

use crate::{helpers::math::div_up, model::post_pub::PostPub};

use super::{
  activity::Activity,
  activity_convertible::ActivityConvertible,
  context::{Context, ContextCollection},
  image::Image,
};

/// Represents an OrderedCollection, containing metadata about how many items
/// are present in the collection, and links to the first and last pages.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedCollection {
  #[serde(rename = "@context")]
  pub context: ContextCollection,
  /// The URL to the first page object
  pub first: String,
  /// The URL to the last page object
  pub last: String,
  /// The URL / UUID for this object
  pub id: String,
  #[serde(rename = "totalItems")]
  pub total_items: i64,
  /// The object type, which should be OrderedCollection in this case
  #[serde(rename = "type")]
  pub object_type: &'static str,
}

/// Represents an OrderedCollectionPage, containing a collection of ordered items
/// and references to the next page and the root OrderedCollection.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedCollectionPage<T> {
  #[serde(rename = "@context")]
  pub context: ContextCollection,
  #[serde(rename = "orderedItems")]
  pub ordered_items: Vec<T>,
  /// The URL / UUID for this object
  pub id: String,
  /// The URL to the OrderedCollection object
  #[serde(rename = "partOf")]
  pub part_of: String,
  /// The URL to the previous page object, if available
  pub prev: Option<String>,
  /// The URL to the next page object, if available
  pub next: Option<String>,
  /// The object type, which should be OrderedCollectionPage in this case
  #[serde(rename = "type")]
  pub object_type: &'static str,
}

impl OrderedCollection {
  pub fn build(subject_url: &str, total_items: i64, page_size: i64) -> OrderedCollection {
    OrderedCollection {
      context: ContextCollection::Single(Context::Plain("https://www.w3.org/ns/activitystreams".to_string())),
      first: format!("{}?page=0&limit={}", subject_url, page_size),
      last: format!(
        "{}?page={}&limit={}",
        subject_url,
        div_up(total_items, page_size),
        page_size
      ),
      id: subject_url.to_string(),
      total_items,
      object_type: "OrderedCollection",
    }
  }
}

impl OrderedCollectionPage<Activity<Image>> {
  pub fn build(
    current_uri: &str,
    collection_uri: &str,
    item_base_uri: &str,
    actor_uri: &str,
    posts: Vec<PostPub>,
    total_items: i64,
    page_size: i64,
    page: i64,
  ) -> OrderedCollectionPage<Activity<Image>> {
    let prev_uri = match page {
      0 => None,
      _ => Some(format!("{}?page={}&page_size={}", &collection_uri, page - 1, page_size)),
    };

    let next_uri = match div_up(total_items, page_size) > page {
      true => Some(format!("{}?page={}&page_size={}", &collection_uri, page + 1, page_size)),
      false => None,
    };

    OrderedCollectionPage {
      context: ContextCollection::Multiple(vec![Context::Plain(
        "https://www.w3.org/ns/activitystreams".to_string(),
      )]),
      ordered_items: posts
        .iter()
        .filter_map(|p| p.to_activity(item_base_uri, actor_uri))
        .collect(),
      id: current_uri.to_string(),
      part_of: collection_uri.to_string(),
      prev: prev_uri,
      next: next_uri,
      object_type: "OrderedCollectionPage",
    }
  }
}

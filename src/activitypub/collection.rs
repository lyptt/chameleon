use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{object::Object, reference::Reference};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct CollectionPageProps {
  #[serde(rename = "partOf", skip_serializing_if = "Option::is_none")]
  pub part_of: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub next: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prev: Option<Reference<Object>>,
  #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
  pub start_index: Option<u32>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct CollectionProps {
  #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
  pub total_items: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub current: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub first: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub last: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub items: Option<Reference<Object>>,
  #[serde(rename = "orderedItems", skip_serializing_if = "Option::is_none")]
  pub ordered_items: Option<Reference<Object>>,
}

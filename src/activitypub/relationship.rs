use super::{object::Object, reference::Reference};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct RelationshipProps {
  #[serde(rename = "attributedTo", skip_serializing_if = "Option::is_none")]
  pub subject: Option<Reference<Object>>,
  #[serde(rename = "attributedTo", skip_serializing_if = "Option::is_none")]
  pub object: Option<Reference<Object>>,
  #[serde(rename = "attributedTo", skip_serializing_if = "Option::is_none")]
  pub relationship: Option<Reference<Object>>,
}

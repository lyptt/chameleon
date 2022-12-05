use super::{object::Object, reference::Reference};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct ActivityProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub actor: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub object: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub result: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub origin: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub instrument: Option<Reference<Object>>,
}

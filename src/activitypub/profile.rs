use super::{object::Object, reference::Reference};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct ProfileProps {
  #[serde(rename = "attributedTo", skip_serializing_if = "Option::is_none")]
  pub describes: Option<Reference<Object>>,
}

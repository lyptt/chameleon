use super::{object::Object, reference::Reference};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct LinkProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub href: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hreflang: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rel: Option<Vec<String>>,
}

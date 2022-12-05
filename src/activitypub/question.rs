use super::{object::Object, reference::Reference};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Display, Debug)]
#[serde(untagged)]
pub enum QuestionClosed {
  Date(DateTime<Utc>),
  Bool(bool),
  Reference(Reference<Object>),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct QuestionProps {
  #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
  pub any_of: Option<Reference<Object>>,
  #[serde(rename = "oneOf", skip_serializing_if = "Option::is_none")]
  pub one_of: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub closed: Option<QuestionClosed>,
}

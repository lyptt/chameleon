use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct TombstoneProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "formerType")]
  pub former_kind: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub deleted: Option<DateTime<Utc>>,
}

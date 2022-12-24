use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct KeyProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner: Option<String>,
  #[serde(rename = "publicKeyPem", skip_serializing_if = "Option::is_none")]
  pub public_key_pem: Option<String>,
}

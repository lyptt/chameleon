use serde::{Deserialize, Serialize};

use super::link::Link;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cc: Option<Vec<String>>,
  pub url: Vec<Link>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  #[serde(rename = "type")]
  pub object_type: &'static str,
}

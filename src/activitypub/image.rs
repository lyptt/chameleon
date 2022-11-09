use serde::{Deserialize, Serialize};

use super::link::Link;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
  pub to: Option<Vec<String>>,
  pub cc: Option<Vec<String>>,
  pub url: Vec<Link>,
  pub name: Option<String>,
  pub content: Option<String>,
  #[serde(rename = "type")]
  pub object_type: &'static str,
}

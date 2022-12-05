use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Note {
  #[serde(rename = "type")]
  pub kind: String,
}

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RdfStringProps {
  pub string: String,
  pub lang: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Display)]
#[serde(untagged)]
pub enum RdfString {
  Raw(String),
  Props(RdfStringProps),
}

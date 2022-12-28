use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Deserialize, Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EventType {
  Unknown,
  Post,
  Boost,
}

impl Default for EventType {
  fn default() -> Self {
    EventType::Unknown
  }
}

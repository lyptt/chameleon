use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Deserialize, Serialize, EnumString, Display, Debug, PartialEq, Eq, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
  Unknown,
  Shadow,
  Unlisted,
  Private,
  FollowersOnly,
  PublicLocal,
  PublicFederated,
}

impl Default for AccessType {
  fn default() -> Self {
    AccessType::Unknown
  }
}

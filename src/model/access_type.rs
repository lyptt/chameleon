use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum::{Display, EnumString};

#[derive(Deserialize, Serialize, EnumString, Display, Type)]
pub enum AccessType {
  Shadow,
  Unlisted,
  Private,
  FriendsOnly,
  PublicLocal,
  PublicFederated,
}

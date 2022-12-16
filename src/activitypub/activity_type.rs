use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Clone, EnumString)]
pub enum ActivityType {
  Accept,
  Add,
  Announce,
  Arrive,
  Block,
  Create,
  Delete,
  Dislike,
  Flag,
  Follow,
  Ignore,
  Invite,
  Join,
  Leave,
  Like,
  Listen,
  Move,
  Offer,
  Question,
  Reject,
  Read,
  Remove,
  TentativeReject,
  TentativeAccept,
  Travel,
  Undo,
  Update,
  View,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::activity_type::ActivityType;

/// Represents an Activity, which is an action a user performs in
/// an ActivityPub environment.
#[derive(Serialize, Deserialize, Debug)]
pub struct Activity<T> {
  pub id: String,
  pub actor: String,
  pub published: DateTime<Utc>,
  pub object: T,
  #[serde(rename = "type")]
  pub activity_type: ActivityType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cc: Option<Vec<String>>,
}

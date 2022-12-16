use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Display, EnumString)]
pub enum QueueJobType {
  Unknown,
  ConvertNewPostImages,
  CreatePostEvents,
  CreatePostEvent,
  CreateBoostEvents,
  CreateBoostEvent,
  DeleteBoostEvents,
  FederateActivityPub,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug, Display, EnumString)]
pub enum OriginDataEntry {
  Raw(String),
  Map(HashMap<String, String>),
}

#[derive(Deserialize, Serialize, TypedBuilder)]
/// Represents an asynchronous job that can be queried by the user.
pub struct QueueJob {
  pub job_id: Uuid,
  pub job_type: QueueJobType,
  #[builder(default, setter(strip_option))]
  pub data: Option<Value>,
  #[builder(default, setter(strip_option))]
  pub origin: Option<String>,
  #[builder(default, setter(strip_option))]
  pub context: Option<Vec<String>>,
  #[builder(default)]
  pub origin_data: Option<HashMap<String, OriginDataEntry>>,
}

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Deserialize, Serialize, EnumString, Display, Debug)]
pub enum QueueJobType {
  ConvertNewPostImages,
  CreatePostEvents,
  CreatePostEvent,
  CreateBoostEvents,
  CreateBoostEvent,
  DeleteBoostEvents,
  FederateActivityPub,
}

#[derive(Deserialize, Serialize)]
/// Represents an asynchronous job that can be queried by the user.
pub struct QueueJob {
  pub job_id: Uuid,
  pub job_type: QueueJobType,
  pub data: Option<Value>,
  pub origin: Option<String>,
  pub context: Option<Vec<String>>,
}

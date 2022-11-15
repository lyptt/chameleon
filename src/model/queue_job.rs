use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

use super::job::JobStatus;

#[derive(Deserialize, Serialize, EnumString, Display, Debug)]
pub enum QueueJobType {
  ConvertNewPostImages,
}

#[derive(Deserialize, Serialize)]
/// Represents an asynchronous job that can be queried by the user.
pub struct QueueJob {
  pub job_id: Uuid,
  pub job_type: QueueJobType,
}

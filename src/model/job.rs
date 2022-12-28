use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize, EnumString, Display, Debug, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
  NotStarted,
  InProgress,
  Done,
  Failed,
}

impl Default for JobStatus {
  fn default() -> Self {
    JobStatus::NotStarted
  }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
/// Represents an asynchronous job that can be queried by the user.
pub struct Job {
  pub job_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub record_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub associated_record_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_by_id: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub status: JobStatus,
  pub failed_count: i32,
}

impl FromRow for Job {
  fn from_row(row: Row) -> Option<Self> {
    Some(Job {
      job_id: row.get("job_id"),
      record_id: row.get("record_id"),
      associated_record_id: row.get("associated_record_id"),
      created_by_id: row.get("created_by_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      status: JobStatus::from_str(row.get("status")).unwrap_or_default(),
      failed_count: row.get("failed_count"),
    })
  }
}

/// Represents a new asynchronous job that can be queried by the user.
pub struct NewJob {
  pub created_by_id: Option<Uuid>,
  pub status: JobStatus,
  pub record_id: Option<Uuid>,
  pub associated_record_id: Option<Uuid>,
}

use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgTypeInfo, Decode, Encode, FromRow, Postgres, Type};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Deserialize, Serialize, EnumString, Display, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
  NotStarted,
  InProgress,
  Done,
  Failed,
}

impl<'r> Encode<'r, Postgres> for JobStatus {
  fn encode_by_ref(
    &self,
    buf: &mut <Postgres as sqlx::database::HasArguments<'r>>::ArgumentBuffer,
  ) -> sqlx::encode::IsNull {
    self.to_string().encode_by_ref(buf)
  }
}

impl<'r> Decode<'r, Postgres> for JobStatus {
  fn decode(value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef) -> Result<Self, sqlx::error::BoxDynError> {
    let s = match value.as_str() {
      Ok(s) => s,
      Err(e) => return Err(e),
    };

    match JobStatus::from_str(s) {
      Ok(t) => Ok(t),
      Err(e) => Err(Box::new(e)),
    }
  }
}

impl Type<Postgres> for JobStatus {
  fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
    PgTypeInfo::with_name("VARCHAR")
  }
}

#[derive(Deserialize, Serialize, Clone, FromRow)]
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

/// Represents a new asynchronous job that can be queried by the user.
pub struct NewJob {
  pub created_by_id: Option<Uuid>,
  pub status: JobStatus,
  pub record_id: Option<Uuid>,
  pub associated_record_id: Option<Uuid>,
}

use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgTypeInfo, Decode, Encode, Error, FromRow, Pool, Postgres, Type};
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
  pub completion_record_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub created_by_id: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub status: JobStatus,
  pub failed_count: i32,
}

/// Represents a new asynchronous job that can be queried by the user.
pub struct NewJob {
  pub job_id: Uuid,
  pub created_by_id: Option<Uuid>,
  pub status: JobStatus,
  pub completion_record_id: Option<Uuid>,
}

impl Job {
  pub async fn fetch_by_id(job_id: &Uuid, pool: &Pool<Postgres>) -> Result<Option<Self>, Error> {
    let user = sqlx::query_as("SELECT * FROM jobs WHERE job_id = $1")
      .bind(job_id)
      .fetch_optional(pool)
      .await?;

    Ok(user)
  }

  pub async fn fetch_optional_by_id(job_id: &Uuid, pool: &Pool<Postgres>) -> Option<Self> {
    match sqlx::query_as("SELECT * FROM jobs WHERE job_id = $1")
      .bind(job_id)
      .fetch_optional(pool)
      .await
    {
      Ok(job) => job,
      Err(_) => None,
    }
  }

  pub async fn create(job: NewJob, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("INSERT INTO jobs (job_id, created_by_id, status, completion_record_id) VALUES ($1, $2, $3, $4)")
      .bind(job.job_id)
      .bind(job.created_by_id)
      .bind(job.status)
      .bind(job.completion_record_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn update(job: &Self, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query(
      "UPDATE jobs SET completion_record_id=$1, status = $2, failed_count = $3, updated_at = now() WHERE job_id = $4",
    )
    .bind(job.completion_record_id)
    .bind(&job.status)
    .bind(job.failed_count)
    .bind(job.job_id)
    .execute(pool)
    .await?;

    Ok(())
  }
}

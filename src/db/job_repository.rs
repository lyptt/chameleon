use std::sync::Arc;

use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::job::{Job, NewJob},
};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait JobRepo {
  async fn fetch_by_id(&self, job_id: &Uuid) -> Result<Option<Job>, LogicErr>;
  async fn fetch_optional_by_id(&self, job_id: &Uuid) -> Option<Job>;
  async fn create(&self, job: NewJob) -> Result<Uuid, LogicErr>;
  async fn update(&self, job: &Job) -> Result<(), LogicErr>;
}

pub type JobPool = Arc<dyn JobRepo + Send + Sync>;

pub struct DbJobRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl JobRepo for DbJobRepo {
  async fn fetch_by_id(&self, job_id: &Uuid) -> Result<Option<Job>, LogicErr> {
    let user = sqlx::query_as("SELECT * FROM jobs WHERE job_id = $1")
      .bind(job_id)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(user)
  }

  async fn fetch_optional_by_id(&self, job_id: &Uuid) -> Option<Job> {
    match sqlx::query_as("SELECT * FROM jobs WHERE job_id = $1")
      .bind(job_id)
      .fetch_optional(&self.db)
      .await
    {
      Ok(job) => job,
      Err(_) => None,
    }
  }

  async fn create(&self, job: NewJob) -> Result<Uuid, LogicErr> {
    let job_id = Uuid::new_v4();

    sqlx::query(
      "INSERT INTO jobs (job_id, created_by_id, status, record_id, associated_record_id) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(job_id)
    .bind(job.created_by_id)
    .bind(job.status)
    .bind(job.record_id)
    .bind(job.associated_record_id)
    .execute(&self.db)
    .await
    .map_err(map_db_err)?;

    Ok(job_id)
  }

  async fn update(&self, job: &Job) -> Result<(), LogicErr> {
    sqlx::query("UPDATE jobs SET record_id = $1, associated_record_id = $2, status = $3, failed_count = $4, updated_at = now() WHERE job_id = $5")
      .bind(job.record_id)
      .bind(job.associated_record_id)
      .bind(&job.status)
      .bind(job.failed_count)
      .bind(job.job_id)
      .execute(&self.db)
      .await.map_err(map_db_err)?;

    Ok(())
  }
}

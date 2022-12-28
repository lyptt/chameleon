use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::job::{Job, NewJob},
};

use super::FromRow;
use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
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
  pub db: Pool,
}

#[async_trait]
impl JobRepo for DbJobRepo {
  async fn fetch_by_id(&self, job_id: &Uuid) -> Result<Option<Job>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT * FROM jobs WHERE job_id = $1", &[&job_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(Job::from_row))
  }

  async fn fetch_optional_by_id(&self, job_id: &Uuid) -> Option<Job> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT * FROM jobs WHERE job_id = $1", &[&job_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(Job::from_row)
  }

  async fn create(&self, job: NewJob) -> Result<Uuid, LogicErr> {
    let job_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "INSERT INTO jobs (job_id, created_by_id, status, record_id, associated_record_id) VALUES ($1, $2, $3, $4, $5)",
      &[
        &job_id,
        &job.created_by_id,
        &job.status.to_string(),
        &job.record_id,
        &job.associated_record_id,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(job_id)
  }

  async fn update(&self, job: &Job) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE jobs SET record_id = $1, associated_record_id = $2, status = $3, failed_count = $4, updated_at = now() WHERE job_id = $5",
      &[
        &job.record_id,
        &job.associated_record_id,
        &job.status.to_string(),
        &job.failed_count,
        &job.job_id,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }
}

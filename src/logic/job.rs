use uuid::Uuid;

use crate::{db::job_repository::JobPool, model::job::Job};

use super::LogicErr;

pub async fn fetch_job(jobs: &JobPool, job_id: &Uuid, user_id: &Option<Uuid>) -> Result<Job, LogicErr> {
  let job = jobs.fetch_by_id(job_id).await?;

  if let Some(job) = job {
    if (job.created_by_id.is_some() && &job.created_by_id == user_id) || job.created_by_id.is_none() {
      Ok(job)
    } else {
      Err(LogicErr::MissingRecord)
    }
  } else {
    Err(LogicErr::MissingRecord)
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use chrono::Utc;
  use mockall::predicate::*;
  use uuid::Uuid;

  use crate::{
    db::job_repository::{JobPool, MockJobRepo},
    logic::{job::fetch_job, LogicErr},
    model::job::{Job, JobStatus},
  };

  #[async_std::test]
  async fn test_get_job_rejects_db_err() {
    let job_id = Uuid::new_v4();
    let user_id = Some(Uuid::new_v4());

    let mut job_repo = MockJobRepo::new();

    job_repo
      .expect_fetch_by_id()
      .times(1)
      .with(eq(job_id))
      .returning(|_| Err(LogicErr::DbError("Error".to_string())));

    let jobs: JobPool = Arc::new(job_repo);

    assert_eq!(
      fetch_job(&jobs, &job_id, &user_id).await,
      Err(LogicErr::DbError("Error".to_string()))
    );
  }

  #[async_std::test]
  async fn test_get_job_rejects_missing_job() {
    let job_id = Uuid::new_v4();
    let user_id = Some(Uuid::new_v4());

    let mut job_repo = MockJobRepo::new();

    job_repo
      .expect_fetch_by_id()
      .times(1)
      .with(eq(job_id))
      .returning(|_| Ok(None));

    let jobs: JobPool = Arc::new(job_repo);

    assert_eq!(fetch_job(&jobs, &job_id, &user_id).await, Err(LogicErr::MissingRecord));
  }

  #[async_std::test]
  async fn test_get_job_rejects_wrong_user() {
    let job_id = Uuid::new_v4();
    let user_id = Some(Uuid::new_v4());

    let mut job_repo = MockJobRepo::new();

    let job = Job {
      job_id,
      record_id: Some(Uuid::new_v4()),
      associated_record_id: Some(Uuid::new_v4()),
      created_by_id: Some(Uuid::new_v4()),
      created_at: Utc::now(),
      updated_at: Utc::now(),
      status: JobStatus::Done,
      failed_count: 0,
    };

    job_repo
      .expect_fetch_by_id()
      .times(1)
      .with(eq(job_id))
      .return_const(Ok(Some(job)));

    let jobs: JobPool = Arc::new(job_repo);

    assert_eq!(fetch_job(&jobs, &job_id, &user_id).await, Err(LogicErr::MissingRecord));
  }

  #[async_std::test]
  async fn test_get_job_succeeds_user_match() {
    let job_id = Uuid::new_v4();
    let user_id = Some(Uuid::new_v4());

    let mut job_repo = MockJobRepo::new();

    let job = Job {
      job_id,
      record_id: Some(Uuid::new_v4()),
      associated_record_id: Some(Uuid::new_v4()),
      created_by_id: user_id,
      created_at: Utc::now(),
      updated_at: Utc::now(),
      status: JobStatus::Done,
      failed_count: 0,
    };
    let job_eq = job.clone();

    job_repo
      .expect_fetch_by_id()
      .times(1)
      .with(eq(job_id))
      .return_const(Ok(Some(job)));

    let jobs: JobPool = Arc::new(job_repo);

    assert_eq!(fetch_job(&jobs, &job_id, &user_id).await, Ok(job_eq));
  }

  #[async_std::test]
  async fn test_get_job_succeeds_no_assoc_user() {
    let job_id = Uuid::new_v4();
    let user_id = Some(Uuid::new_v4());

    let mut job_repo = MockJobRepo::new();

    let job = Job {
      job_id,
      record_id: Some(Uuid::new_v4()),
      associated_record_id: Some(Uuid::new_v4()),
      created_by_id: None,
      created_at: Utc::now(),
      updated_at: Utc::now(),
      status: JobStatus::Done,
      failed_count: 0,
    };
    let job_eq = job.clone();

    job_repo
      .expect_fetch_by_id()
      .times(1)
      .with(eq(job_id))
      .return_const(Ok(Some(job)));

    let jobs: JobPool = Arc::new(job_repo);

    assert_eq!(fetch_job(&jobs, &job_id, &user_id).await, Ok(job_eq));
  }
}

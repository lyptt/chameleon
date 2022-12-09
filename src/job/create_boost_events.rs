use uuid::Uuid;

use crate::{
  db::{follow_repository::FollowPool, job_repository::JobPool},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn create_boost_events(
  jobs: &JobPool,
  follows: &FollowPool,
  job_id: Uuid,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let user_id = match job.created_by_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("User ID not found for job".to_string())),
  };

  let followers = follows.fetch_user_followers(&user_id).await.unwrap_or_default();

  for follower in followers {
    let job_id = jobs
      .create(NewJob {
        created_by_id: Some(user_id),
        status: JobStatus::NotStarted,
        record_id: Some(post_id),
        associated_record_id: Some(follower.user_id),
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob {
      job_id,
      job_type: QueueJobType::CreateBoostEvent,
      data: None,
      origin: None,
      context: None,
    };

    queue.send_job(job).await?;
  }

  Ok(())
}

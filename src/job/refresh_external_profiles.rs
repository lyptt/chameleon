use crate::{
  db::{job_repository::JobPool, user_repository::UserPool},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn refresh_external_profiles(users: &UserPool, jobs: &JobPool, queue: &Queue) -> Result<(), LogicErr> {
  let refreshing_users = users.fetch_outdated_external_users().await?;
  for user in refreshing_users {
    let job_id = jobs
      .create(NewJob {
        created_by_id: None,
        status: JobStatus::NotStarted,
        record_id: Some(user),
        associated_record_id: None,
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::RefreshExternalProfile)
      .build();

    queue.send_job(job).await?;
  }

  Ok(())
}

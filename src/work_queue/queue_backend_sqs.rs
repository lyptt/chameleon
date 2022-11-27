use super::queue::{Queue, QueueBackend};
use crate::{
  aws::clients::SQS_CLIENT,
  cdn::cdn_store::Cdn,
  helpers::api::map_ext_err,
  job::delegate_job,
  logic::LogicErr,
  model::{
    job::{Job, JobStatus},
    queue_job::QueueJob,
  },
  settings::SETTINGS,
};
use async_trait::async_trait;
use log::error;
use sqlx::{Pool, Postgres};

pub struct QueueBackendSQS {}

impl QueueBackendSQS {}

#[async_trait]
impl QueueBackend for QueueBackendSQS {
  async fn send_job(&self, queue_job: QueueJob) -> Result<(), LogicErr> {
    let message_body = match serde_json::to_string(&queue_job) {
      Ok(body) => body,
      Err(err) => return Err(LogicErr::InternalError(err.to_string())),
    };

    SQS_CLIENT
      .get()
      .unwrap()
      .send_message()
      .queue_url(&SETTINGS.queue.work_queue)
      .message_body(message_body)
      .send()
      .await
      .map_err(map_ext_err)?;

    Ok(())
  }

  async fn receive_jobs(&self, db: Pool<Postgres>, cdn: &Cdn, queue: &Queue) -> Result<(), LogicErr> {
    let rcv_message_output = SQS_CLIENT
      .get()
      .unwrap()
      .receive_message()
      .wait_time_seconds(20)
      .visibility_timeout(60 * 5)
      .queue_url(&SETTINGS.queue.work_queue)
      .send()
      .await
      .map_err(map_ext_err)?;

    for message in rcv_message_output.messages.unwrap_or_default() {
      let receipt_handle = match message.receipt_handle() {
        Some(handle) => handle,
        None => {
          continue;
        }
      };

      let body = match message.body() {
        Some(body) => body,
        None => {
          continue;
        }
      };

      let queue_job: QueueJob = match serde_json::from_str(body) {
        Ok(queue_job) => queue_job,
        Err(err) => {
          error!(
            "Failed to deserialize queue_job with id {}: {}",
            message.message_id().unwrap_or_default(),
            err.to_string()
          );
          continue;
        }
      };

      let mut job = match Job::fetch_optional_by_id(&queue_job.job_id, &db).await {
        Some(job) => job,
        None => {
          error!(
            "Job not found in db with id {}",
            message.message_id().unwrap_or_default(),
          );
          continue;
        }
      };

      job.status = JobStatus::InProgress;
      match Job::update(&job, &db).await {
        Ok(_) => {}
        Err(err) => {
          error!(
            "Failed to update job in db with id {}: {}",
            message.message_id().unwrap_or_default(),
            err.to_string()
          );
          continue;
        }
      }

      let result = delegate_job(&queue_job, &db, cdn, queue).await;

      match result {
        Ok(()) => {
          job.status = JobStatus::Done;
          match Job::update(&job, &db).await {
            Ok(_) => {}
            Err(err) => {
              error!(
                "Failed to update job in db with id {}: {}",
                message.message_id().unwrap_or_default(),
                err.to_string()
              );
              continue;
            }
          }

          match SQS_CLIENT
            .get()
            .unwrap()
            .delete_message()
            .receipt_handle(receipt_handle)
            .queue_url(&SETTINGS.queue.work_queue)
            .send()
            .await
          {
            Ok(_) => {}
            Err(err) => {
              error!(
                "Failed to delete completed queue_job with id {}: {}",
                message.message_id().unwrap_or_default(),
                err.to_string()
              );
            }
          }
        }
        Err(err) => {
          error!(
            "Failed to run queue_job with id {}: {}",
            message.message_id().unwrap_or_default(),
            err.to_string()
          );

          job.failed_count += 1;
          job.status = JobStatus::NotStarted;
          match Job::update(&job, &db).await {
            Ok(_) => {}
            Err(err) => {
              error!(
                "Failed to update job in db with id {}: {}",
                message.message_id().unwrap_or_default(),
                err.to_string()
              );
            }
          }
        }
      }
    }

    Ok(())
  }
}

use super::queue::{Queue, QueueBackend};
use crate::{cdn::cdn_store::Cdn, logic::LogicErr, model::queue_job::QueueJob};
use async_trait::async_trait;
use log::warn;
use sqlx::{Pool, Postgres};

pub struct QueueBackendNoop {}

impl QueueBackendNoop {}

#[async_trait]
impl QueueBackend for QueueBackendNoop {
  async fn send_job(&self, queue_job: QueueJob) -> Result<(), LogicErr> {
    warn!(
      "Queue backend is 'noop'. Queue job {} will be ignored.",
      queue_job.job_id
    );
    Ok(())
  }

  async fn receive_jobs(&self, _db: Pool<Postgres>, _cdn: &Cdn, _queue: &Queue) -> Result<(), LogicErr> {
    warn!("Queue backend is 'noop'. No queue jobs will be processed.");
    Ok(())
  }
}

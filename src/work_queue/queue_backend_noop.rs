use super::queue::{Queue, QueueBackend};
use crate::{cdn::cdn_store::Cdn, db::repositories::Repositories, logic::LogicErr, model::queue_job::QueueJob};
use async_trait::async_trait;
use log::warn;

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

  async fn receive_jobs(&self, _cdn: &Cdn, _queue: &Queue, _repositories: &Repositories) -> Result<(), LogicErr> {
    warn!("Queue backend is 'noop'. No queue jobs will be processed.");
    Ok(())
  }
}

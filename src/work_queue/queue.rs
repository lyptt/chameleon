use crate::{
  cdn::cdn_store::Cdn,
  db::repositories::Repositories,
  logic::LogicErr,
  model::queue_job::QueueJob,
  settings::{AppQueueBackend, SETTINGS},
};

use async_trait::async_trait;
use std::result::Result;

use super::{
  queue_backend_noop::QueueBackendNoop, queue_backend_rabbitmq::QueueBackendRabbitMQ,
  queue_backend_sqs::QueueBackendSQS,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait QueueBackend {
  async fn send_job(&self, job: QueueJob) -> Result<(), LogicErr>;
  async fn receive_jobs(&self, cdn: &Cdn, queue: &Queue, repositories: &Repositories) -> Result<(), LogicErr>;
}

pub struct Queue {
  imp: Box<dyn QueueBackend + Send + Sync + 'static>,
}

impl Queue {
  pub fn new() -> Queue {
    match SETTINGS.queue.queue_backend {
      AppQueueBackend::Sqs => Queue {
        imp: Box::new(QueueBackendSQS {}),
      },
      AppQueueBackend::RabbitMQ => Queue {
        imp: Box::new(QueueBackendRabbitMQ {}),
      },
      AppQueueBackend::Noop => Queue {
        imp: Box::new(QueueBackendNoop {}),
      },
    }
  }

  #[cfg(test)]
  pub fn new_inner(inner: Box<dyn QueueBackend + Sync + Send>) -> Queue {
    Queue { imp: inner }
  }

  pub async fn send_job(&self, job: QueueJob) -> Result<(), LogicErr> {
    self.imp.send_job(job).await
  }

  pub async fn receive_jobs(&self, cdn: &Cdn, queue: &Queue, repositories: &Repositories) -> Result<(), LogicErr> {
    self.imp.receive_jobs(cdn, queue, repositories).await
  }
}

use crate::{
  cdn::cdn_store::Cdn,
  logic::LogicErr,
  model::queue_job::QueueJob,
  settings::{AppQueueBackend, SETTINGS},
};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use std::result::Result;

use super::{queue_backend_rabbitmq::QueueBackendRabbitMQ, queue_backend_sqs::QueueBackendSQS};

#[async_trait]
pub trait QueueBackend {
  async fn send_job(&self, job: QueueJob) -> Result<(), LogicErr>;
  async fn receive_jobs(&self, db: Pool<Postgres>, cdn: &Cdn, queue: &Queue) -> Result<(), LogicErr>;
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
    }
  }

  pub async fn send_job(&self, job: QueueJob) -> Result<(), LogicErr> {
    self.imp.send_job(job).await
  }

  pub async fn receive_jobs(&self, db: Pool<Postgres>, cdn: &Cdn, queue: &Queue) -> Result<(), LogicErr> {
    self.imp.receive_jobs(db, cdn, queue).await
  }
}

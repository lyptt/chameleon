use super::queue::{Queue, QueueBackend};
use crate::{
  cdn::cdn_store::Cdn,
  db::{job_repository::JobPool, repositories::Repositories},
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
use futures_util::StreamExt;
use lapin::{
  message::Delivery,
  options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions},
  types::FieldTable,
  BasicProperties,
};
use log::error;
use uuid::Uuid;

use crate::rabbitmq::clients::RABBITMQ_WORK_CHANNEL;

pub struct QueueBackendRabbitMQ {}

impl QueueBackendRabbitMQ {
  async fn reject_job(job: Job, queue_job: &Delivery, jobs: &JobPool, err_msg: &str, err: LogicErr) {
    error!("{}: {}: {}", err_msg, job.job_id, err.to_string());

    let mut db_job = job.clone();
    db_job.failed_count += 1;
    db_job.status = JobStatus::Failed;

    let mut should_requeue = true;

    match jobs.update(&db_job).await {
      Ok(_) => {
        if cfg!(debug_assertions) {
          // Give ourselves more opportunity to diagnose job issues in debug builds
          if db_job.failed_count > 49 {
            should_requeue = false;
          }
        } else if db_job.failed_count > 4 {
          should_requeue = false;
        }
      }
      Err(err) => {
        error!(
          "Failed to update job in db with id {}: {}",
          db_job.job_id,
          err.to_string()
        );
        should_requeue = false;
      }
    }

    let opts = BasicNackOptions {
      requeue: should_requeue,
      ..Default::default()
    };
    let result = queue_job.nack(opts).await;
    if let Err(err) = result {
      error!("Failed to reject queue_job: {}: {}", db_job.job_id, err.to_string());
    }
  }
}

#[async_trait]
impl QueueBackend for QueueBackendRabbitMQ {
  async fn send_job(&self, queue_job: QueueJob) -> Result<(), LogicErr> {
    let data = match serde_json::to_vec(&queue_job) {
      Ok(data) => data,
      Err(err) => return Err(map_ext_err(err)),
    };

    RABBITMQ_WORK_CHANNEL
      .get()
      .unwrap()
      .basic_publish(
        "",
        &SETTINGS.queue.work_queue,
        BasicPublishOptions::default(),
        &data,
        BasicProperties::default(),
      )
      .await
      .map_err(map_ext_err)?;

    Ok(())
  }

  async fn receive_jobs(&self, cdn: &Cdn, queue: &Queue, repositories: &Repositories) -> Result<(), LogicErr> {
    let tag = Uuid::new_v4().to_string();
    let mut consumer = match RABBITMQ_WORK_CHANNEL
      .get()
      .unwrap()
      .basic_consume(
        &SETTINGS.queue.work_queue,
        &tag,
        BasicConsumeOptions::default(),
        FieldTable::default(),
      )
      .await
    {
      Ok(c) => c,
      Err(err) => return Err(map_ext_err(err)),
    };

    while let Some(delivery) = consumer.next().await {
      let job = match delivery {
        Ok(delivery) => delivery,
        Err(error) => {
          error!("Failed to consume queue message {}", error);
          continue;
        }
      };

      let queue_job: QueueJob = match serde_json::from_slice(&job.data) {
        Ok(job) => job,
        Err(err) => {
          error!("Failed to deserialize queue_job: {}", err.to_string());
          let opts = BasicNackOptions {
            requeue: false,
            ..Default::default()
          };
          let result = job.nack(opts).await;
          if let Err(err) = result {
            error!("Failed to reject queue_job: {}", err.to_string());
          }
          continue;
        }
      };

      let mut db_job = match repositories.jobs.fetch_optional_by_id(&queue_job.job_id).await {
        Some(job) => job,
        None => {
          error!("Job not found in db with id {}", queue_job.job_id,);
          let opts = BasicNackOptions {
            requeue: false,
            ..Default::default()
          };
          let result = job.nack(opts).await;
          if let Err(err) = result {
            error!("Failed to reject queue_job: {}: {}", queue_job.job_id, err.to_string());
          }
          continue;
        }
      };

      db_job.status = JobStatus::InProgress;
      match repositories.jobs.update(&db_job).await {
        Ok(_) => {}
        Err(err) => {
          QueueBackendRabbitMQ::reject_job(
            db_job,
            &job,
            &repositories.jobs,
            "Failed to update job in db with id",
            map_ext_err(err),
          )
          .await;
          continue;
        }
      }

      let result = delegate_job(&queue_job, repositories, cdn, queue).await;

      match result {
        Ok(()) => {
          db_job.status = JobStatus::Done;
          match repositories.jobs.update(&db_job).await {
            Ok(_) => {}
            Err(err) => {
              QueueBackendRabbitMQ::reject_job(
                db_job,
                &job,
                &repositories.jobs,
                "Failed to update job in db with id",
                map_ext_err(err),
              )
              .await;
              continue;
            }
          }
        }
        Err(err) => {
          QueueBackendRabbitMQ::reject_job(
            db_job,
            &job,
            &repositories.jobs,
            "Failed to run queue_job with id",
            map_ext_err(err),
          )
          .await;
          continue;
        }
      }

      if let Err(err) = job.ack(BasicAckOptions::default()).await {
        QueueBackendRabbitMQ::reject_job(
          db_job,
          &job,
          &repositories.jobs,
          "Failed to ack queue_job with id",
          map_ext_err(err),
        )
        .await;
        continue;
      }
    }

    Ok(())
  }
}

use sqlx::{Pool, Postgres};

use crate::{
  cdn::cdn_store::Cdn,
  logic::LogicErr,
  model::queue_job::{QueueJob, QueueJobType},
  work_queue::queue::Queue,
};

pub mod convert_new_post_images;
pub mod create_event;
pub mod create_events;

pub async fn delegate_job(queue_job: &QueueJob, db: &Pool<Postgres>, cdn: &Cdn, queue: &Queue) -> Result<(), LogicErr> {
  match queue_job.job_type {
    QueueJobType::ConvertNewPostImages => {
      convert_new_post_images::convert_new_post_images(queue_job.job_id, &db, cdn, queue).await
    }
    QueueJobType::CreateEvents => create_events::create_events(queue_job.job_id, &db, queue).await,
    QueueJobType::CreateEvent => create_event::create_event(queue_job.job_id, &db).await,
  }
}

use sqlx::{Pool, Postgres};

use crate::{
  cdn::cdn_store::Cdn,
  logic::LogicErr,
  model::queue_job::{QueueJob, QueueJobType},
  work_queue::queue::Queue,
};

mod convert_new_post_images;
mod create_boost_event;
mod create_boost_events;
mod create_post_event;
mod create_post_events;
mod delete_boost_events;

pub async fn delegate_job(queue_job: &QueueJob, db: &Pool<Postgres>, cdn: &Cdn, queue: &Queue) -> Result<(), LogicErr> {
  match queue_job.job_type {
    QueueJobType::ConvertNewPostImages => {
      convert_new_post_images::convert_new_post_images(queue_job.job_id, db, cdn, queue).await
    }
    QueueJobType::CreatePostEvents => create_post_events::create_post_events(queue_job.job_id, db, queue).await,
    QueueJobType::CreatePostEvent => create_post_event::create_post_event(queue_job.job_id, db).await,
    QueueJobType::CreateBoostEvents => create_boost_events::create_boost_events(queue_job.job_id, db, queue).await,
    QueueJobType::CreateBoostEvent => create_boost_event::create_boost_event(queue_job.job_id, db).await,
    QueueJobType::DeleteBoostEvents => delete_boost_events::delete_boost_events(queue_job.job_id, db).await,
  }
}

use crate::{
  cdn::cdn_store::Cdn,
  db::repositories::Repositories,
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
mod federate_activitypub;

pub async fn delegate_job(
  queue_job: &QueueJob,
  repositories: &Repositories,
  cdn: &Cdn,
  queue: &Queue,
) -> Result<(), LogicErr> {
  match queue_job.job_type {
    QueueJobType::ConvertNewPostImages => {
      convert_new_post_images::convert_new_post_images(
        queue_job.job_id,
        &repositories.jobs,
        &repositories.posts,
        cdn,
        queue,
      )
      .await
    }
    QueueJobType::CreatePostEvents => {
      create_post_events::create_post_events(
        &repositories.jobs,
        &repositories.posts,
        &repositories.events,
        &repositories.follows,
        queue_job.job_id,
        queue,
      )
      .await
    }
    QueueJobType::CreatePostEvent => {
      create_post_event::create_post_event(
        &repositories.jobs,
        &repositories.posts,
        &repositories.events,
        queue_job.job_id,
      )
      .await
    }
    QueueJobType::CreateBoostEvents => {
      create_boost_events::create_boost_events(&repositories.jobs, &repositories.follows, queue_job.job_id, queue).await
    }
    QueueJobType::CreateBoostEvent => {
      create_boost_event::create_boost_event(
        queue_job.job_id,
        &repositories.jobs,
        &repositories.posts,
        &repositories.events,
      )
      .await
    }
    QueueJobType::DeleteBoostEvents => {
      delete_boost_events::delete_boost_events(queue_job.job_id, &repositories.jobs, &repositories.events).await
    }
    QueueJobType::FederateActivityPub => {
      federate_activitypub::federate_activitypub(
        queue_job.job_id,
        &queue_job.data,
        &queue_job.origin,
        &queue_job.origin_data,
        &queue_job.context,
        repositories,
        cdn,
        queue,
      )
      .await
    }
    QueueJobType::Unknown => Err(LogicErr::Unimplemented),
  }
}

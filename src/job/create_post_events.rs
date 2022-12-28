use log::warn;
use uuid::Uuid;

use crate::{
  db::{
    event_repository::EventPool, follow_repository::FollowPool, job_repository::JobPool, post_repository::PostPool,
    user_orbit_repository::UserOrbitPool,
  },
  helpers::api::{map_db_err, map_ext_err},
  logic::LogicErr,
  model::{
    event::NewEvent,
    event_type::EventType,
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn create_post_events(
  jobs: &JobPool,
  posts: &PostPool,
  events: &EventPool,
  follows: &FollowPool,
  user_orbits: &UserOrbitPool,
  job_id: Uuid,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  let user_id = match job.created_by_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("User ID not found for job".to_string())),
  };

  let post = posts.fetch_by_id(&post_id).await?;

  let own_event = NewEvent {
    source_user_id: user_id,
    target_user_id: None,
    visibility: post.visibility.clone(),
    post_id: Some(post_id),
    like_id: None,
    comment_id: None,
    event_type: EventType::Post,
  };

  if let Err(err) = events.create_event(own_event).await.map_err(map_ext_err) {
    warn!("Failed to create user's own event for new post: {}", err);
  }

  let followers = follows.fetch_user_followers(&user_id).await.unwrap_or_default();

  for follower in followers {
    let job_id = jobs
      .create(NewJob {
        created_by_id: Some(user_id),
        status: JobStatus::NotStarted,
        record_id: Some(post_id),
        associated_record_id: Some(follower.user_id),
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::CreatePostEvent)
      .build();

    queue.send_job(job).await?;
  }

  if let Some(orbit_id) = post.orbit_id {
    let users = user_orbits.fetch_orbit_user_ids(&orbit_id).await?;

    for user in users {
      let job_id = jobs
        .create(NewJob {
          created_by_id: Some(user_id),
          status: JobStatus::NotStarted,
          record_id: Some(post_id),
          associated_record_id: Some(user),
        })
        .await
        .map_err(map_db_err)?;

      let job = QueueJob::builder()
        .job_id(job_id)
        .job_type(QueueJobType::CreatePostEvent)
        .build();

      queue.send_job(job).await?;
    }
  }

  Ok(())
}

use log::warn;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  helpers::api::{map_db_err, map_ext_err},
  logic::LogicErr,
  model::{
    event::{Event, NewEvent},
    event_type::EventType,
    follow::Follow,
    job::{Job, JobStatus, NewJob},
    post::Post,
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn create_events(job_id: Uuid, db: &Pool<Postgres>, queue: &Queue) -> Result<(), LogicErr> {
  let job = match Job::fetch_optional_by_id(&job_id, db).await {
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

  let visibility = match Post::fetch_visibility_by_id(&post_id, &db).await {
    Some(v) => v,
    None => return Err(LogicErr::InternalError("Visibility not found for post".to_string())),
  };

  let own_event = NewEvent {
    source_user_id: user_id.clone(),
    target_user_id: None,
    visibility: visibility.clone(),
    post_id: Some(post_id.clone()),
    like_id: None,
    comment_id: None,
    event_type: EventType::Post,
  };

  if let Err(err) = Event::create_event(own_event, &db).await.map_err(map_ext_err) {
    warn!("Failed to create user's own event for new post: {}", err);
  }

  let followers = Follow::fetch_user_followers(&user_id, &db).await.unwrap_or_default();

  for follower in followers {
    let job_id = Job::create(
      NewJob {
        created_by_id: Some(user_id.clone()),
        status: JobStatus::NotStarted,
        record_id: Some(post_id.clone()),
        associated_record_id: Some(follower.user_id.clone()),
      },
      db,
    )
    .await
    .map_err(map_db_err)?;

    let job = QueueJob {
      job_id,
      job_type: QueueJobType::CreateEvent,
    };

    queue.send_job(job).await?;
  }

  Ok(())
}

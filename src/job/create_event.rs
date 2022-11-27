use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  helpers::api::map_ext_err,
  logic::LogicErr,
  model::{
    event::{Event, NewEvent},
    event_type::EventType,
    job::Job,
    post::Post,
  },
};

pub async fn create_event(job_id: Uuid, db: &Pool<Postgres>) -> Result<(), LogicErr> {
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

  let target_user_id = match job.associated_record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Target User ID not found for job".to_string())),
  };

  let visibility = match Post::fetch_visibility_by_id(&post_id, &db).await {
    Some(v) => v,
    None => return Err(LogicErr::InternalError("Visibility not found for post".to_string())),
  };

  let own_event = NewEvent {
    source_user_id: user_id.clone(),
    target_user_id: Some(target_user_id),
    visibility: visibility.clone(),
    post_id: Some(post_id.clone()),
    like_id: None,
    comment_id: None,
    event_type: EventType::Post,
  };

  Event::create_event(own_event, &db).await.map_err(map_ext_err)
}

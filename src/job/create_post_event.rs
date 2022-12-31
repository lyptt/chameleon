use uuid::Uuid;

use crate::{
  db::{event_repository::EventPool, job_repository::JobPool, post_repository::PostPool, user_repository::UserPool},
  federation::activitypub::{federate_ext, FederateExtAction},
  helpers::api::map_ext_err,
  logic::LogicErr,
  model::{event::NewEvent, event_type::EventType},
};

pub async fn create_post_event(
  jobs: &JobPool,
  posts: &PostPool,
  events: &EventPool,
  users: &UserPool,
  job_id: Uuid,
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

  let target_user_id = match job.associated_record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Target User ID not found for job".to_string())),
  };

  let visibility = match posts.fetch_visibility_by_id(&post_id).await {
    Some(v) => v,
    None => return Err(LogicErr::InternalError("Visibility not found for post".to_string())),
  };

  let dest_user = match users.fetch_by_id(&target_user_id).await {
    Ok(user) => user,
    Err(err) => return Err(err),
  };

  if dest_user.fediverse_uri.starts_with("http") {
    let user = match users.fetch_by_id(&user_id).await {
      Ok(user) => user,
      Err(err) => return Err(err),
    };

    return federate_ext(FederateExtAction::CreatePost(&post_id), &user, &dest_user, posts).await;
  }

  // TODO: Make sure event doesn't exist first

  let own_event = NewEvent {
    source_user_id: user_id,
    target_user_id: Some(target_user_id),
    visibility: visibility.clone(),
    post_id: Some(post_id),
    like_id: None,
    comment_id: None,
    event_type: EventType::Post,
  };

  events.create_event(own_event).await.map_err(map_ext_err)
}

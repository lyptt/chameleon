use uuid::Uuid;

use crate::{
  db::{event_repository::EventPool, job_repository::JobPool},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::event_type::EventType,
};

pub async fn delete_boost_events(job_id: Uuid, jobs: &JobPool, events: &EventPool) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let user_id = match job.created_by_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("User not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  events
    .delete_post_events(&post_id, &user_id, EventType::Boost)
    .await
    .map_err(map_db_err)
}

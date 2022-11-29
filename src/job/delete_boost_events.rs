use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{event::Event, event_type::EventType, job::Job},
};

pub async fn delete_boost_events(job_id: Uuid, db: &Pool<Postgres>) -> Result<(), LogicErr> {
  let job = match Job::fetch_optional_by_id(&job_id, db).await {
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

  Event::delete_post_events(&post_id, &user_id, EventType::Boost, db)
    .await
    .map_err(map_db_err)
}

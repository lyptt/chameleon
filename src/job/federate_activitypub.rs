use serde_json::Value;
use uuid::Uuid;

use crate::{cdn::cdn_store::Cdn, db::repositories::Repositories, logic::LogicErr, work_queue::queue::Queue};

pub async fn federate_activitypub(
  job_id: Uuid,
  data: &Option<Value>,
  origin: &Option<String>,
  context: &Option<Vec<String>>,
  repositories: &Repositories,
  cdn: &Cdn,
  queue: &Queue,
) -> Result<(), LogicErr> {
  println!("{} {:?} {:?} {:?}", job_id, data, origin, context);
  Ok(())
}

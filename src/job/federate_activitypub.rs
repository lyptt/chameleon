use serde_json::Value;
use std::collections::HashMap;

use crate::{
  activitypub::document::{ActivityPubDocument, RawActivityPubDocument},
  db::repositories::Repositories,
  federation::activitypub::federate,
  logic::LogicErr,
  model::queue_job::OriginDataEntry,
  work_queue::queue::Queue,
};

pub async fn federate_activitypub(
  data: &Option<Value>,
  origin_data: &Option<HashMap<String, OriginDataEntry>>,
  repositories: &Repositories,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let doc: RawActivityPubDocument = match data.to_owned() {
    Some(value) => match serde_json::from_value(value) {
      Ok(doc) => doc,
      Err(err) => return Err(LogicErr::InvalidOperation(err.to_string())),
    },
    None => return Err(LogicErr::MissingRecord),
  };

  let doc = match ActivityPubDocument::from(doc) {
    Ok(doc) => doc,
    Err(err) => return Err(LogicErr::InvalidOperation(err.to_string())),
  };

  federate(
    doc,
    origin_data,
    &repositories.users,
    &repositories.follows,
    &repositories.posts,
    &repositories.likes,
    &repositories.jobs,
    queue,
  )
  .await
}

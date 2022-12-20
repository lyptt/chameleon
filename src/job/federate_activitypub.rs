use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::{
  activitypub::document::{ActivityPubDocument, RawActivityPubDocument},
  cdn::cdn_store::Cdn,
  db::repositories::Repositories,
  federation::activitypub::federate,
  logic::LogicErr,
  model::queue_job::OriginDataEntry,
  work_queue::queue::Queue,
};

pub async fn federate_activitypub(
  _job_id: Uuid,
  data: &Option<Value>,
  origin_data: &Option<HashMap<String, OriginDataEntry>>,
  repositories: &Repositories,
  _cdn: &Cdn,
  _queue: &Queue,
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

  federate(doc, origin_data, &repositories.users).await
}

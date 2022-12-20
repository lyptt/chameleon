use super::{
  actor::federate_actor,
  note::federate_create_note,
  util::{deref_activitypub_ref, determine_activity_visibility},
};
use crate::{
  activitypub::{activity_type::ActivityType, document::ActivityPubDocument, object::ObjectType},
  db::{follow_repository::FollowPool, job_repository::JobPool, post_repository::PostPool, user_repository::UserPool},
  helpers::core::unwrap_or_fail,
  logic::LogicErr,
  model::queue_job::OriginDataEntry,
  net::http_sig::verify_http_signature,
  settings::SETTINGS,
  work_queue::queue::Queue,
};

use std::{collections::HashMap, str::FromStr};

pub async fn federate(
  doc: ActivityPubDocument,
  origin_data: &Option<HashMap<String, OriginDataEntry>>,
  users: &UserPool,
  follows: &FollowPool,
  posts: &PostPool,
  jobs: &JobPool,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let kind = match unwrap_or_fail(doc.object.kind.as_ref().map(|v| ActivityType::from_str(v))) {
    Ok(kind) => kind,
    Err(err) => return Err(err),
  };

  let actor_user = match federate_actor(&doc.object.actor, users).await {
    Ok(user) => user,
    Err(err) => return Err(err),
  };

  if SETTINGS.app.secure && !verify_http_signature(origin_data, &actor_user.public_key) {
    return Err(LogicErr::UnauthorizedError);
  }

  let activity_visibility = match determine_activity_visibility(&doc.object.to, &actor_user) {
    Some(v) => v,
    None => return Err(LogicErr::InvalidData),
  };

  let activity = match &doc.object.activity {
    Some(ac) => ac,
    None => return Err(LogicErr::InvalidData),
  };

  let object = match deref_activitypub_ref(&activity.object).await {
    Some(obj) => obj,
    None => return Err(LogicErr::InvalidData),
  };

  let object_type = match &object.kind {
    Some(v) => match ObjectType::from_str(v) {
      Ok(t) => t,
      Err(_) => return Err(LogicErr::InvalidData),
    },
    None => return Err(LogicErr::InvalidData),
  };

  match object_type {
    ObjectType::Note => match kind {
      ActivityType::Create => {
        federate_create_note(object, actor_user, activity_visibility, follows, posts, jobs, queue).await
      }
      _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
    },
    _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
  }
}

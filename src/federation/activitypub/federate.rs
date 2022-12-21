use super::{
  actor::federate_actor,
  note::{federate_create_note, federate_delete_note, federate_like_note, federate_unlike_note, federate_update_note},
  person::{federate_create_follow, federate_remove_follow},
  util::{
    activitypub_ref_to_uri_opt, deref_activitypub_ref, determine_activity_target, determine_activity_visibility,
    ActivityTarget,
  },
};
use crate::{
  activitypub::{activity_type::ActivityType, document::ActivityPubDocument, object::ObjectType},
  db::{
    follow_repository::FollowPool, job_repository::JobPool, like_repository::LikePool, post_repository::PostPool,
    user_repository::UserPool,
  },
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
  likes: &LikePool,
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

  let activity_visibility = determine_activity_visibility(&doc.object.to, &actor_user);

  let activity = match &doc.object.activity {
    Some(ac) => ac,
    None => return Err(LogicErr::InvalidData),
  };

  let object = match deref_activitypub_ref(&activity.object).await {
    Some(obj) => obj,
    None => return Err(LogicErr::InvalidData),
  };

  let target = activitypub_ref_to_uri_opt(&activity.target);

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
        let activity_visibility = match activity_visibility {
          Some(v) => v,
          None => return Err(LogicErr::InvalidData),
        };

        federate_create_note(object, actor_user, activity_visibility, follows, posts, jobs, queue).await
      }
      ActivityType::Update => {
        let activity_visibility = match activity_visibility {
          Some(v) => v,
          None => return Err(LogicErr::InvalidData),
        };

        federate_update_note(object, actor_user, activity_visibility, posts).await
      }
      ActivityType::Like => federate_like_note(object, actor_user, posts, likes).await,
      ActivityType::Remove => match determine_activity_target(target) {
        ActivityTarget::PostLikes(target) => federate_unlike_note(target, actor_user, posts, likes).await,
        ActivityTarget::Post(target) => federate_delete_note(target, actor_user, posts).await,
        _ => Err(LogicErr::InvalidData),
      },
      ActivityType::Delete => match determine_activity_target(target) {
        ActivityTarget::PostLikes(target) => federate_unlike_note(target, actor_user, posts, likes).await,
        ActivityTarget::Post(target) => federate_delete_note(target, actor_user, posts).await,
        _ => Err(LogicErr::InvalidData),
      },
      _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
    },
    ObjectType::Person => match kind {
      ActivityType::Follow => federate_create_follow(object, actor_user, follows, users).await,
      ActivityType::Remove => match determine_activity_target(target) {
        ActivityTarget::UserFollowers(target) => federate_remove_follow(target, actor_user, follows, users).await,
        _ => Err(LogicErr::InvalidData),
      },
      ActivityType::Delete => match determine_activity_target(target) {
        ActivityTarget::UserFollowers(target) => federate_remove_follow(target, actor_user, follows, users).await,
        _ => Err(LogicErr::InvalidData),
      },
      _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
    },
    _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
  }
}

use uuid::Uuid;

use super::{
  actor::federate_actor,
  note::{
    federate_create_note, federate_ext_create_note, federate_like_note, federate_unlike_note, federate_update_note,
  },
  object::federate_delete_remote_object,
  person::{federate_create_follow, federate_remove_follow},
  util::{
    activitypub_ref_to_uri_opt, deref_activitypub_ref, determine_activity_target, determine_activity_visibility,
    send_activitypub_object, ActivityTarget, FederateResult,
  },
};
use crate::{
  activitypub::{
    activity::ActivityProps,
    activity_type::ActivityType,
    document::ActivityPubDocument,
    object::{Object, ObjectType},
    reference::Reference,
  },
  db::{
    follow_repository::FollowPool, job_repository::JobPool, like_repository::LikePool,
    post_attachment_repository::PostAttachmentPool, post_repository::PostPool, user_repository::UserPool,
  },
  helpers::core::unwrap_or_fail,
  logic::LogicErr,
  model::{queue_job::OriginDataEntry, user::User},
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
  post_attachments: &PostAttachmentPool,
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

  let result = match object_type {
    ObjectType::Note => match kind {
      ActivityType::Create => {
        let activity_visibility = match activity_visibility {
          Some(v) => v,
          None => return Err(LogicErr::InvalidData),
        };

        federate_create_note(
          object,
          &actor_user,
          activity_visibility,
          follows,
          posts,
          jobs,
          post_attachments,
          queue,
        )
        .await
      }
      ActivityType::Update => {
        let activity_visibility = match activity_visibility {
          Some(v) => v,
          None => return Err(LogicErr::InvalidData),
        };

        federate_update_note(object, &actor_user, activity_visibility, posts).await
      }
      ActivityType::Like => federate_like_note(object, &actor_user, posts, likes).await,
      ActivityType::Remove => match determine_activity_target(target) {
        ActivityTarget::PostLikes(target) => federate_unlike_note(target, &actor_user, posts, likes).await,
        ActivityTarget::Unknown(target) => {
          federate_delete_remote_object(target, &actor_user, object_type, origin_data, posts, users, likes).await
        }
        _ => Err(LogicErr::InvalidData),
      },
      ActivityType::Delete => match determine_activity_target(target) {
        ActivityTarget::PostLikes(target) => federate_unlike_note(target, &actor_user, posts, likes).await,
        ActivityTarget::Unknown(target) => {
          federate_delete_remote_object(target, &actor_user, object_type, origin_data, posts, users, likes).await
        }
        _ => Err(LogicErr::InvalidData),
      },
      _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
    },
    ObjectType::Person => match kind {
      ActivityType::Follow => federate_create_follow(object, &actor_user, follows, users).await,
      ActivityType::Remove => match determine_activity_target(target) {
        ActivityTarget::UserFollowers(target) => federate_remove_follow(target, &actor_user, follows, users).await,
        ActivityTarget::Unknown(target) => {
          federate_delete_remote_object(target, &actor_user, object_type, origin_data, posts, users, likes).await
        }
        _ => Err(LogicErr::InvalidData),
      },
      ActivityType::Delete => match determine_activity_target(target) {
        ActivityTarget::UserFollowers(target) => federate_remove_follow(target, &actor_user, follows, users).await,
        ActivityTarget::Unknown(target) => {
          federate_delete_remote_object(target, &actor_user, object_type, origin_data, posts, users, likes).await
        }
        _ => Err(LogicErr::InvalidData),
      },
      _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
    },
    ObjectType::Tombstone => match object.id {
      Some(id) => match id.starts_with(&SETTINGS.server.api_root_fqdn) {
        true => match determine_activity_target(Some(id)) {
          ActivityTarget::UserFollowers(target) => federate_remove_follow(target, &actor_user, follows, users).await,
          ActivityTarget::PostLikes(target) => federate_unlike_note(target, &actor_user, posts, likes).await,
          _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
        },
        false => federate_delete_remote_object(id, &actor_user, object_type, origin_data, posts, users, likes).await,
      },
      None => Err(LogicErr::InvalidData),
    },
    _ => Err(LogicErr::InternalError("Unimplemented".to_string())),
  };

  match result {
    Ok(result) => {
      let (activity_type, actor) = match result {
        FederateResult::None => return Ok(()),
        FederateResult::Accept(actor) => (ActivityType::Accept, actor),
        FederateResult::TentativeAccept(actor) => (ActivityType::TentativeAccept, actor),
        FederateResult::Ignore(actor) => (ActivityType::Ignore, actor),
        FederateResult::Reject(actor) => (ActivityType::Reject, actor),
        FederateResult::TentativeReject(actor) => (ActivityType::TentativeReject, actor),
      };

      let response_object = Object::builder()
        .kind(Some(activity_type.to_string()))
        .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
        .actor(Some(Reference::Remote(format!(
          "{}{}",
          SETTINGS.server.api_fqdn, actor.fediverse_uri
        ))))
        .activity(Some(
          ActivityProps::builder()
            .object(Some(Reference::Embedded(Box::new(doc.object))))
            .build(),
        ))
        .build();

      let doc = ActivityPubDocument::new(response_object);

      let response_uri = match actor_user.ext_apub_inbox_uri {
        Some(uri) => uri,
        None => return Ok(()),
      };

      send_activitypub_object(&response_uri, doc, &actor).await
    }
    Err(err) => Err(err),
  }
}

pub enum FederateExtAction<'a> {
  CreatePost(&'a Uuid),
  UpdatePost(&'a Uuid),
}

pub async fn federate_ext<'a>(
  action: FederateExtAction<'a>,
  actor: &User,
  dest_actor: &User,
  posts: &PostPool,
) -> Result<(), LogicErr> {
  match action {
    FederateExtAction::CreatePost(post_id) => federate_ext_create_note(post_id, actor, dest_actor, posts).await,
    FederateExtAction::UpdatePost(_) => Err(LogicErr::Unimplemented),
  }
}

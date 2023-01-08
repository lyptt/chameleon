use chrono::Utc;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity::ActivityProps,
    activity_convertible::ActivityConvertible,
    activity_type::ActivityType,
    document::ActivityPubDocument,
    object::{Object, ObjectType},
    reference::Reference,
    tombstone::TombstoneProps,
  },
  db::{follow_repository::FollowPool, orbit_repository::OrbitPool, user_repository::UserPool},
  logic::LogicErr,
  model::user::User,
  settings::SETTINGS,
};

use super::{
  util::{send_activitypub_object, FederateResult},
  FederateExtActor,
};

pub async fn federate_create_follow(
  activity_object: Object,
  actor: &User,
  follows: &FollowPool,
  users: &UserPool,
) -> Result<FederateResult, LogicErr> {
  let uri = match activity_object.id {
    Some(uri) => match uri.starts_with(&SETTINGS.server.api_fqdn) {
      true => uri.replace(&SETTINGS.server.api_fqdn, ""),
      false => uri,
    },
    None => return Err(LogicErr::MissingRecord),
  };

  let followed_user = match users.fetch_by_fediverse_uri(&uri).await {
    Some(user) => user,
    None => return Err(LogicErr::MissingRecord),
  };

  if followed_user.is_external {
    return Err(LogicErr::MissingRecord);
  }

  if !follows.user_follows_user(&actor.user_id, &followed_user.user_id).await {
    follows.create_follow(&actor.user_id, &followed_user.user_id).await?;
  }

  Ok(FederateResult::Accept((
    followed_user.fediverse_uri,
    followed_user.private_key,
  )))
}

pub async fn federate_remove_follow(
  target: String,
  actor: &User,
  follows: &FollowPool,
  users: &UserPool,
) -> Result<FederateResult, LogicErr> {
  let uri = match target.starts_with(&SETTINGS.server.api_fqdn) {
    true => target.replace(&SETTINGS.server.api_fqdn, ""),
    false => target,
  };

  let unfollowed_user = match users.fetch_by_fediverse_uri(&uri).await {
    Some(user) => user,
    None => return Err(LogicErr::MissingRecord),
  };

  if unfollowed_user.is_external {
    return Err(LogicErr::MissingRecord);
  }

  follows.delete_follow(&actor.user_id, &unfollowed_user.user_id).await?;

  Ok(FederateResult::Accept((
    unfollowed_user.fediverse_uri,
    unfollowed_user.private_key,
  )))
}

pub async fn federate_ext_create_follow(actor: &User, following_actor: &FederateExtActor) -> Result<(), LogicErr> {
  let following_actor = match following_actor {
    FederateExtActor::Person(actor) => actor,
    _ => return Err(LogicErr::MissingRecord),
  };

  let obj = match following_actor.to_object(&actor.fediverse_uri) {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Follow.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(obj))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &following_actor.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

pub async fn federate_ext_remove_follow(actor: &User, unfollowing_actor: &FederateExtActor) -> Result<(), LogicErr> {
  let unfollowing_actor = match unfollowing_actor {
    FederateExtActor::Person(actor) => actor,
    _ => return Err(LogicErr::MissingRecord),
  };

  let followers_uri = match &unfollowing_actor.ext_apub_followers_uri {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Delete.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(
          Object::builder()
            .id(Some(followers_uri.to_owned()))
            .kind(Some(ObjectType::Tombstone.to_string()))
            .tombstone(Some(
              TombstoneProps::builder()
                .former_kind(Some(ObjectType::Person.to_string()))
                .deleted(Some(Utc::now()))
                .build(),
            ))
            .build(),
        ))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &unfollowing_actor.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

pub async fn federate_ext_join_group(actor: &User, joining_orbit: &Uuid, orbits: &OrbitPool) -> Result<(), LogicErr> {
  let orbit = match orbits.fetch_orbit(joining_orbit).await? {
    Some(orbit) => orbit,
    None => return Err(LogicErr::MissingRecord),
  };

  let obj = match orbit.to_object(&actor.fediverse_uri) {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Follow.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(obj))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &orbit.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

pub async fn federate_ext_leave_group(actor: &User, leaving_orbit: &Uuid, orbits: &OrbitPool) -> Result<(), LogicErr> {
  let orbit = match orbits.fetch_orbit(leaving_orbit).await? {
    Some(orbit) => orbit,
    None => return Err(LogicErr::MissingRecord),
  };

  let orbit_followers_uri = match orbit.ext_apub_followers_uri {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Delete.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(
          Object::builder()
            .id(Some(orbit_followers_uri.to_owned()))
            .kind(Some(ObjectType::Tombstone.to_string()))
            .tombstone(Some(
              TombstoneProps::builder()
                .former_kind(Some(ObjectType::Person.to_string()))
                .deleted(Some(Utc::now()))
                .build(),
            ))
            .build(),
        ))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &orbit.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

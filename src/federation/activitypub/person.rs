use crate::{
  activitypub::object::Object,
  db::{follow_repository::FollowPool, user_repository::UserPool},
  logic::LogicErr,
  model::user::User,
  settings::SETTINGS,
};

use super::util::FederateResult;

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

  Ok(FederateResult::Accept(followed_user))
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

  Ok(FederateResult::Accept(unfollowed_user))
}

use crate::{
  activitypub::object::Object,
  db::{follow_repository::FollowPool, user_repository::UserPool},
  logic::LogicErr,
  model::user::User,
  settings::SETTINGS,
};

pub async fn federate_create_follow(
  activity_object: Object,
  actor: User,
  follows: &FollowPool,
  users: &UserPool,
) -> Result<(), LogicErr> {
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

  follows.create_follow(&actor.user_id, &followed_user.user_id).await?;

  Ok(())
}

pub async fn federate_remove_follow(
  target: String,
  actor: User,
  follows: &FollowPool,
  users: &UserPool,
) -> Result<(), LogicErr> {
  let unfollowed_user = match users.fetch_by_fediverse_uri(&target).await {
    Some(user) => user,
    None => return Err(LogicErr::MissingRecord),
  };

  follows.delete_follow(&actor.user_id, &unfollowed_user.user_id).await
}

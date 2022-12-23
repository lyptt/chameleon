use std::collections::HashMap;

use crate::{
  activitypub::object::ObjectType,
  db::{like_repository::LikePool, post_repository::PostPool, user_repository::UserPool},
  logic::LogicErr,
  model::{queue_job::OriginDataEntry, user::User},
  net::http_sig::extract_http_signature_origin,
  settings::SETTINGS,
};

use super::util::FederateResult;

/// Invoked when a Tombstone object is delivered to us, or we're deleting or removing any remote object.
pub async fn federate_delete_remote_object(
  target: String,
  actor: &User,
  object_type: ObjectType,
  origin_data: &Option<HashMap<String, OriginDataEntry>>,
  posts: &PostPool,
  users: &UserPool,
  _likes: &LikePool,
) -> Result<FederateResult, LogicErr> {
  if SETTINGS.app.secure {
    let origin = match extract_http_signature_origin(origin_data) {
      Some(origin) => origin,
      None => return Err(LogicErr::MissingRecord),
    };

    if !target.starts_with(&format!("http://{}", origin)) && !target.starts_with(&format!("https://{}", origin)) {
      return Err(LogicErr::MissingRecord);
    }
  }

  if target.starts_with(&SETTINGS.server.api_root_fqdn) {
    return Err(LogicErr::MissingRecord);
  }

  match object_type {
    ObjectType::Note => {
      posts.delete_post_from_uri(&target, &actor.user_id).await?;
      return Ok(FederateResult::None);
    }
    ObjectType::Profile => {
      users.delete_user_from_uri(&target).await?;
      return Ok(FederateResult::None);
    }
    _ => {}
  };

  // HACK:
  // Mastodon doesn't send the 'formerType' prop, so we're forced to scour our whole DB looking for the URI
  // it's deleted.
  //
  // Mastodon also sends a Delete activity for the Tombstone, which makes no sense as it's not deleting
  // a Tombstone, it's deleting an object, so we're forced to assume that any notification of Tombstone
  // means that an object is going to be deleted somewhere somehow with whatever URI it sends us.
  posts.delete_post_from_uri(&target, &actor.user_id).await?;
  users.delete_user_from_uri(&target).await?;

  Ok(FederateResult::None)
}

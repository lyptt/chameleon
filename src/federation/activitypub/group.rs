use crate::{
  activitypub::object::Object,
  db::{orbit_repository::OrbitPool, user_orbit_repository::UserOrbitPool},
  logic::LogicErr,
  model::user::User,
  settings::SETTINGS,
};

use super::util::FederateResult;

pub async fn federate_create_member(
  activity_object: Object,
  actor: &User,
  user_orbits: &UserOrbitPool,
  orbits: &OrbitPool,
) -> Result<FederateResult, LogicErr> {
  let uri = match activity_object.id {
    Some(uri) => match uri.starts_with(&SETTINGS.server.api_fqdn) {
      true => uri.replace(&SETTINGS.server.api_fqdn, ""),
      false => uri,
    },
    None => return Err(LogicErr::MissingRecord),
  };

  let target_orbit = match orbits.fetch_by_fediverse_uri(&uri).await {
    Some(user) => user,
    None => return Err(LogicErr::MissingRecord),
  };

  if target_orbit.is_external {
    return Err(LogicErr::MissingRecord);
  }

  if !user_orbits
    .user_is_member(&actor.user_id, &target_orbit.orbit_id)
    .await?
  {
    user_orbits
      .create_user_orbit(&target_orbit.orbit_id, &actor.user_id)
      .await?;
  }

  Ok(FederateResult::Accept((
    target_orbit.fediverse_uri.to_owned(),
    target_orbit.private_key.to_string(),
  )))
}

pub async fn federate_remove_member(
  target: String,
  actor: &User,
  user_orbits: &UserOrbitPool,
  orbits: &OrbitPool,
) -> Result<FederateResult, LogicErr> {
  let uri = match target.starts_with(&SETTINGS.server.api_fqdn) {
    true => target.replace(&SETTINGS.server.api_fqdn, ""),
    false => target,
  };

  let target_orbit = match orbits.fetch_by_fediverse_uri(&uri).await {
    Some(user) => user,
    None => return Err(LogicErr::MissingRecord),
  };

  if target_orbit.is_external {
    return Err(LogicErr::MissingRecord);
  }

  user_orbits
    .delete_user_orbit(&target_orbit.orbit_id, &actor.user_id)
    .await?;

  Ok(FederateResult::Accept((
    target_orbit.fediverse_uri.to_owned(),
    target_orbit.private_key.to_string(),
  )))
}

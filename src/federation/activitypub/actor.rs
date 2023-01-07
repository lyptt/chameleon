use std::str::FromStr;

use chrono::Utc;
use http::Uri;
use uuid::Uuid;

use crate::{
  activitypub::{object::Object, orbit::OrbitProps, rdf_string::RdfString, reference::Reference},
  db::{orbit_repository::OrbitPool, user_repository::UserPool},
  logic::LogicErr,
  model::{orbit::Orbit, user::User},
  settings::SETTINGS,
};

use super::util::{activitypub_ref_to_uri_opt, deref_activitypub_ref};

async fn query_activitypub_user_ref(obj_ref: &Option<Reference<Object>>, users: &UserPool) -> Option<User> {
  let uri = match obj_ref {
    Some(a) => match a {
      Reference::Embedded(obj) => obj.id.clone(),
      Reference::Remote(uri) => Some(uri.to_owned()),
      Reference::Mixed(_) => None,
      Reference::Map(_) => None,
    },
    None => None,
  };

  let uri = match uri {
    Some(uri) => uri,
    None => return None,
  };

  let uri = match uri.starts_with(&SETTINGS.server.api_fqdn) {
    true => uri.replace(&SETTINGS.server.api_fqdn, ""),
    false => uri,
  };

  users.fetch_by_fediverse_uri(&uri).await
}

async fn query_activitypub_orbit_ref(obj_ref: &Option<Reference<Object>>, orbits: &OrbitPool) -> Option<Orbit> {
  let uri = match obj_ref {
    Some(a) => match a {
      Reference::Embedded(obj) => obj.id.clone(),
      Reference::Remote(uri) => Some(uri.to_owned()),
      Reference::Mixed(_) => None,
      Reference::Map(_) => None,
    },
    None => None,
  };

  let uri = match uri {
    Some(uri) => uri,
    None => return None,
  };

  let uri = match uri.starts_with(&SETTINGS.server.api_fqdn) {
    true => uri.replace(&SETTINGS.server.api_fqdn, ""),
    false => uri,
  };

  orbits.fetch_by_fediverse_uri(&uri).await
}

pub async fn federate_user_actor(actor_ref: &Option<Reference<Object>>, users: &UserPool) -> Result<User, LogicErr> {
  if let Some(user) = query_activitypub_user_ref(actor_ref, users).await {
    return Ok(user);
  }

  let actor_obj = match deref_activitypub_ref(actor_ref).await {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let actor = match &actor_obj.actors {
    Some(obj) => obj.to_owned(),
    None => return Err(LogicErr::MissingRecord),
  };

  let handle = match actor.preferred_username {
    Some(handle) => handle,
    None => return Err(LogicErr::InvalidData),
  };

  let following_uri = match activitypub_ref_to_uri_opt(&actor.following) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let followers_uri = match activitypub_ref_to_uri_opt(&actor.followers) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let inbox_uri = match activitypub_ref_to_uri_opt(&actor.inbox) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let outbox_uri = match activitypub_ref_to_uri_opt(&actor.outbox) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let public_key = match actor_obj.key {
    Some(k) => match k.public_key_pem {
      Some(k) => k,
      None => return Err(LogicErr::InvalidData),
    },
    None => return Err(LogicErr::InvalidData),
  };

  let fediverse_uri = match actor_obj.id {
    Some(id) => match Uri::from_str(&id) {
      Ok(uri) => uri,
      Err(_) => return Err(LogicErr::InvalidData),
    },
    None => return Err(LogicErr::InvalidData),
  };

  let fediverse_uri_host = match fediverse_uri.host() {
    Some(v) => v,
    None => return Err(LogicErr::InvalidData),
  };

  let avatar_url = match deref_activitypub_ref(&actor_obj.icon).await {
    Some(obj) => activitypub_ref_to_uri_opt(&obj.url),
    None => None,
  };

  let user = User {
    user_id: Uuid::new_v4(),
    fediverse_id: format!("@{}@{}", handle, fediverse_uri_host),
    handle,
    fediverse_uri: fediverse_uri.to_string(),
    avatar_url,
    email: None,
    password_hash: None,
    is_external: true,
    // TODO: Support pulling these in from profile attachments like Mastodon
    url_1: None,
    url_2: None,
    url_3: None,
    url_4: None,
    url_5: None,
    url_1_title: None,
    url_2_title: None,
    url_3_title: None,
    url_4_title: None,
    url_5_title: None,
    intro_md: None,
    intro_html: None,
    private_key: "".to_string(),
    public_key,
    ext_apub_followers_uri: Some(followers_uri),
    ext_apub_following_uri: Some(following_uri),
    ext_apub_inbox_uri: Some(inbox_uri),
    ext_apub_outbox_uri: Some(outbox_uri),
    created_at: Utc::now(),
  };

  users.create_from(&user).await
}

pub async fn federate_update_user_actor(
  actor_ref: &Option<Reference<Object>>,
  users: &UserPool,
) -> Result<User, LogicErr> {
  let mut user = match query_activitypub_user_ref(actor_ref, users).await {
    Some(user) => user,
    None => return federate_user_actor(actor_ref, users).await,
  };

  let actor_obj = match deref_activitypub_ref(actor_ref).await {
    Some(obj) => obj,
    None => return Ok(user),
  };

  let actor = match &actor_obj.actors {
    Some(obj) => obj.to_owned(),
    None => return Ok(user),
  };

  let handle = match actor.preferred_username {
    Some(handle) => handle,
    None => return Ok(user),
  };

  let following_uri = match activitypub_ref_to_uri_opt(&actor.following) {
    Some(uri) => uri,
    None => return Ok(user),
  };

  let followers_uri = match activitypub_ref_to_uri_opt(&actor.followers) {
    Some(uri) => uri,
    None => return Ok(user),
  };

  let inbox_uri = match activitypub_ref_to_uri_opt(&actor.inbox) {
    Some(uri) => uri,
    None => return Ok(user),
  };

  let outbox_uri = match activitypub_ref_to_uri_opt(&actor.outbox) {
    Some(uri) => uri,
    None => return Ok(user),
  };

  let public_key = match actor_obj.key {
    Some(k) => match k.public_key_pem {
      Some(k) => k,
      None => return Ok(user),
    },
    None => return Ok(user),
  };

  let fediverse_uri = match actor_obj.id {
    Some(id) => match Uri::from_str(&id) {
      Ok(uri) => uri,
      Err(_) => return Ok(user),
    },
    None => return Ok(user),
  };

  let fediverse_uri_host = match fediverse_uri.host() {
    Some(v) => v,
    None => return Ok(user),
  };

  let avatar_url = match deref_activitypub_ref(&actor_obj.icon).await {
    Some(obj) => activitypub_ref_to_uri_opt(&obj.url),
    None => None,
  };

  user.fediverse_id = format!("@{}@{}", handle, fediverse_uri_host);
  user.avatar_url = avatar_url;
  user.public_key = public_key;
  user.ext_apub_followers_uri = Some(followers_uri);
  user.ext_apub_following_uri = Some(following_uri);
  user.ext_apub_inbox_uri = Some(inbox_uri);
  user.ext_apub_outbox_uri = Some(outbox_uri);

  users.update_from(&user).await
}

pub async fn federate_orbit_group(
  group_ref: &Option<Reference<Object>>,
  orbits: &OrbitPool,
) -> Result<Orbit, LogicErr> {
  if let Some(orbit) = query_activitypub_orbit_ref(group_ref, orbits).await {
    return Ok(orbit);
  }

  let actor_obj = match deref_activitypub_ref(group_ref).await {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let actor = match &actor_obj.actors {
    Some(obj) => obj.to_owned(),
    None => return Err(LogicErr::MissingRecord),
  };

  let orbit = match &actor_obj.orbit {
    Some(obj) => obj.to_owned(),
    None => OrbitProps::builder().build(),
  };

  let name = match actor_obj.name {
    Some(name) => name,
    None => return Err(LogicErr::InvalidData),
  };

  let shortcode = match actor.preferred_username {
    Some(shortcode) => shortcode,
    None => return Err(LogicErr::InvalidData),
  };

  let public_key = match actor_obj.key {
    Some(k) => match k.public_key_pem {
      Some(k) => k,
      None => return Err(LogicErr::InvalidData),
    },
    None => return Err(LogicErr::InvalidData),
  };

  let summary_html = match actor_obj.summary {
    Some(summary) => match summary {
      RdfString::Raw(content) => content,
      RdfString::Props(props) => props.string,
    },
    None => "".to_string(),
  };

  let summary_md = match orbit.summary_md {
    Some(summary) => summary,
    None => "".to_string(),
  };

  let followers_uri = match activitypub_ref_to_uri_opt(&actor.followers) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let inbox_uri = match activitypub_ref_to_uri_opt(&actor.inbox) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let outbox_uri = match activitypub_ref_to_uri_opt(&actor.outbox) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let fediverse_uri = match actor_obj.id {
    Some(id) => match Uri::from_str(&id) {
      Ok(uri) => uri,
      Err(_) => return Err(LogicErr::InvalidData),
    },
    None => return Err(LogicErr::InvalidData),
  };

  let avatar_url = match deref_activitypub_ref(&actor_obj.icon).await {
    Some(obj) => activitypub_ref_to_uri_opt(&obj.url),
    None => None,
  };

  let banner_url = match deref_activitypub_ref(&actor_obj.image).await {
    Some(obj) => activitypub_ref_to_uri_opt(&obj.url),
    None => None,
  };

  let orbit = Orbit {
    orbit_id: Uuid::new_v4(),
    created_at: Utc::now(),
    updated_at: Utc::now(),
    shortcode,
    name,
    description_md: summary_md,
    description_html: summary_html,
    avatar_uri: avatar_url,
    banner_uri: banner_url,
    uri: fediverse_uri.to_string(),
    fediverse_uri: fediverse_uri.to_string(),
    public_key,
    private_key: "".to_string(),
    is_external: true,
    ext_apub_inbox_uri: Some(inbox_uri),
    ext_apub_outbox_uri: Some(outbox_uri),
    ext_apub_followers_uri: Some(followers_uri),
  };

  orbits.create_from(&orbit).await
}

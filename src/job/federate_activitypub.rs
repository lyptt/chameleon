use std::{collections::HashMap, str::FromStr};

use serde_json::Value;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_type::ActivityType,
    document::{ActivityPubDocument, RawActivityPubDocument},
    object::Object,
    reference::Reference,
  },
  cdn::cdn_store::Cdn,
  db::{repositories::Repositories, user_repository::UserPool},
  helpers::core::unwrap_or_fail,
  logic::LogicErr,
  model::{access_type::AccessType, queue_job::OriginDataEntry, user::User},
  net::http_sig::verify_http_signature,
  work_queue::queue::Queue,
};

fn activitypub_ref_to_uri(obj_ref: &Reference<Object>) -> Option<String> {
  match obj_ref {
    Reference::Embedded(_) => None,
    Reference::Remote(uri) => Some(uri.to_owned()),
    Reference::Mixed(_) => None,
    Reference::Map(_) => None,
  }
}

fn activitypub_ref_to_uri_opt(obj_ref: &Option<Reference<Object>>) -> Option<String> {
  match obj_ref {
    Some(obj_ref) => activitypub_ref_to_uri(obj_ref),
    None => None,
  }
}

async fn fetch_activitypub_object(obj_ref: &str) -> Option<Object> {
  todo!();
}

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

  users.fetch_by_fediverse_uri(&uri).await
}

async fn deref_activitypub_ref(obj_ref: &Option<Reference<Object>>) -> Option<Object> {
  match obj_ref {
    Some(a) => match a {
      Reference::Embedded(obj) => Some((**obj).clone()),
      Reference::Remote(uri) => fetch_activitypub_object(uri).await,
      Reference::Mixed(_) => None,
      Reference::Map(_) => None,
    },
    None => None,
  }
}

async fn federate_actor(actor_ref: &Option<Reference<Object>>, users: &UserPool) -> Result<User, LogicErr> {
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
    Some(id) => id,
    None => return Err(LogicErr::InvalidData),
  };

  let avatar_url = match deref_activitypub_ref(&actor_obj.icon).await {
    Some(obj) => activitypub_ref_to_uri_opt(&obj.url),
    None => None,
  };

  let user = User {
    user_id: Uuid::new_v4(),
    fediverse_id: format!("acctext:{}", handle),
    handle,
    fediverse_uri,
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
  };

  users.create_from(&user).await
}

fn determine_activity_visibility(to: &Option<Reference<Object>>, author: &User) -> Option<AccessType> {
  let objs = match to {
    Some(obj_ref) => match obj_ref {
      Reference::Embedded(_) => return None,
      Reference::Remote(uri) => vec![uri.to_owned()],
      Reference::Mixed(multi) => multi.iter().flat_map(activitypub_ref_to_uri).collect(),
      Reference::Map(_) => return None,
    },
    None => return None,
  };

  if objs.contains(&"https://www.w3.org/ns/activitystreams#Public".to_string()) {
    return Some(AccessType::PublicFederated);
  }

  let followers_uri = author.ext_apub_followers_uri.clone().unwrap_or_default();

  if objs.iter().any(|v| *v == followers_uri) {
    return Some(AccessType::FollowersOnly);
  }

  Some(AccessType::Unlisted)
}

pub async fn federate_activitypub(
  job_id: Uuid,
  data: &Option<Value>,
  origin: &Option<String>,
  origin_data: &Option<HashMap<String, OriginDataEntry>>,
  context: &Option<Vec<String>>,
  repositories: &Repositories,
  cdn: &Cdn,
  queue: &Queue,
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

  let kind = match unwrap_or_fail(doc.object.kind.map(|v| ActivityType::from_str(&v))) {
    Ok(kind) => kind,
    Err(err) => return Err(err),
  };

  let actor_user = match federate_actor(&doc.object.actor, &repositories.users).await {
    Ok(user) => user,
    Err(err) => return Err(err),
  };

  if !verify_http_signature(origin_data, &actor_user.public_key) {
    return Err(LogicErr::UnauthorizedError);
  }

  let activity_visibility = match determine_activity_visibility(&doc.object.to, &actor_user) {
    Some(v) => v,
    None => return Err(LogicErr::InvalidData),
  };

  match kind {
    ActivityType::Accept => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Add => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Announce => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Arrive => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Block => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Create => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Delete => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Dislike => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Flag => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Follow => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Ignore => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Invite => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Join => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Leave => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Like => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Listen => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Move => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Offer => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Question => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Reject => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Read => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Remove => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::TentativeReject => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::TentativeAccept => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Travel => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Undo => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::Update => Err(LogicErr::InternalError("Unimplemented".to_string())),
    ActivityType::View => Err(LogicErr::InternalError("Unimplemented".to_string())),
  }
}

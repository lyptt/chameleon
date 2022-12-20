use uuid::Uuid;

use crate::{
  activitypub::{object::Object, reference::Reference},
  db::user_repository::UserPool,
  logic::LogicErr,
  model::user::User,
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

  users.fetch_by_fediverse_uri(&uri).await
}

pub async fn federate_actor(actor_ref: &Option<Reference<Object>>, users: &UserPool) -> Result<User, LogicErr> {
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

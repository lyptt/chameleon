use std::collections::HashMap;

use crate::{
  activitypub::{activity_convertible::ActivityConvertible, actor::ActorProps, object::Object, reference::Reference},
  settings::SETTINGS,
};

use super::user::User;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
/// Represents account details for a user. This should only be returned to the user
/// this data refers to, and only from an authenticated session for that particular
/// user.
pub struct UserAccountPub {
  pub user_id: Uuid,
  pub fediverse_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub handle: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub avatar_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_1: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_2: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_3: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_4: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_5: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_1_title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_2_title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_3_title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_4_title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url_5_title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub intro_md: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub intro_html: Option<String>,
}

impl From<User> for UserAccountPub {
  fn from(u: User) -> Self {
    UserAccountPub {
      user_id: u.user_id,
      fediverse_id: u.fediverse_id,
      handle: u.handle,
      avatar_url: u.avatar_url,
      email: u.email,
      url_1: u.url_1,
      url_2: u.url_2,
      url_3: u.url_3,
      url_4: u.url_4,
      url_5: u.url_5,
      url_1_title: u.url_1_title,
      url_2_title: u.url_2_title,
      url_3_title: u.url_3_title,
      url_4_title: u.url_4_title,
      url_5_title: u.url_5_title,
      intro_md: u.intro_md,
      intro_html: u.intro_html,
    }
  }
}

impl ActivityConvertible for UserAccountPub {
  fn to_object(&self, _actor: &str) -> Object {
    let handle = self.handle.clone().unwrap_or_default();

    let id = format!("{}/users/{}", SETTINGS.server.api_fqdn, &handle);
    let public_inbox_uri = format!("{}/federate/activitypub/shared-inbox", SETTINGS.server.api_fqdn);
    let inbox_uri = format!("{}/federate/activitypub/inbox/{}", SETTINGS.server.api_fqdn, &handle);
    let outbox_uri = format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, &handle);
    let followers_uri = format!("{}/api/users/{}/followers", SETTINGS.server.api_fqdn, &handle);
    let following_uri = format!("{}/api/users/{}/following", SETTINGS.server.api_fqdn, &handle);
    let icon = self.avatar_url.clone().map(|avatar_url| {
      Reference::Embedded(Box::new(
        Object::builder()
          .kind(Some("Image".to_string()))
          .media_type(Some("image/jpeg".to_string()))
          .url(Some(Reference::Remote(avatar_url)))
          .build(),
      ))
    });
    let mut endpoints = HashMap::new();
    endpoints.insert("sharedInbox".to_string(), serde_json::Value::String(public_inbox_uri));

    Object::builder()
      .id(Some(id.clone()))
      .kind(Some("Person".to_string()))
      .icon(icon)
      .url(Some(Reference::Remote(id)))
      .actors(Some(
        ActorProps::builder()
          .endpoints(Some(Reference::Map(endpoints)))
          .followers(Some(Reference::Remote(followers_uri)))
          .following(Some(Reference::Remote(following_uri)))
          .inbox(Some(Reference::Remote(inbox_uri)))
          .outbox(Some(Reference::Remote(outbox_uri)))
          .preferred_username(Some(handle))
          .build(),
      ))
      .build()
  }
}

#[cfg(test)]
mod tests {
  use crate::model::{user::User, user_account_pub::UserAccountPub};

  use sqlx::types::Uuid;
  use std::str::FromStr;

  #[test]
  fn test_from_user() {
    let user = User {
      user_id: Uuid::from_str("ae1481a5-2eb7-4c52-93c3-e95839578dce").unwrap(),
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: Some("a".to_string()),
      avatar_url: None,
      email: Some("b".to_string()),
      password_hash: Some("c".to_string()),
      is_external: true,
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
    };
    let user_cmp = user.clone();

    let val: UserAccountPub = user.into();

    assert_eq!(val.user_id, user_cmp.user_id);
    assert_eq!(val.fediverse_id, user_cmp.fediverse_id);
    assert_eq!(val.handle, user_cmp.handle);
    assert_eq!(val.email, user_cmp.email);
  }
}

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible, actor::ActorProps, key::KeyProps, object::Object, reference::Reference,
  },
  db::FromRow,
  helpers::api::relative_to_absolute_uri,
  settings::SETTINGS,
};

use super::webfinger::{WebfingerRecord, WebfingerRecordLink};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
  pub user_id: Uuid,
  pub fediverse_id: String,
  pub handle: String,
  pub fediverse_uri: String,
  pub avatar_url: Option<String>,
  pub email: Option<String>,
  pub password_hash: Option<String>,
  pub is_external: bool,
  pub url_1: Option<String>,
  pub url_2: Option<String>,
  pub url_3: Option<String>,
  pub url_4: Option<String>,
  pub url_5: Option<String>,
  pub url_1_title: Option<String>,
  pub url_2_title: Option<String>,
  pub url_3_title: Option<String>,
  pub url_4_title: Option<String>,
  pub url_5_title: Option<String>,
  pub intro_md: Option<String>,
  pub intro_html: Option<String>,
  pub private_key: String,
  pub public_key: String,
  pub ext_apub_followers_uri: Option<String>,
  pub ext_apub_following_uri: Option<String>,
  pub ext_apub_inbox_uri: Option<String>,
  pub ext_apub_outbox_uri: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl User {
  pub fn to_webfinger(&self) -> WebfingerRecord {
    WebfingerRecord {
      aliases: Some(vec![WebfingerRecordLink::build_self_uri(&self.handle)]),
      subject: self.fediverse_id.replacen('@', "acct:", 1),
      links: [
        WebfingerRecordLink::build_self_link(&self.handle),
        WebfingerRecordLink::build_profile_page_link(&self.handle),
        WebfingerRecordLink::build_feed_link(&self.handle),
      ]
      .into(),
    }
  }
}

impl FromRow for User {
  fn from_row(row: Row) -> Option<Self> {
    Some(User {
      user_id: row.get("user_id"),
      fediverse_id: row.get("fediverse_id"),
      handle: row.get("handle"),
      fediverse_uri: row.get("fediverse_uri"),
      avatar_url: row.get("avatar_url"),
      email: row.get("email"),
      password_hash: row.get("password_hash"),
      is_external: row.get("is_external"),
      url_1: row.get("url_1"),
      url_2: row.get("url_2"),
      url_3: row.get("url_3"),
      url_4: row.get("url_4"),
      url_5: row.get("url_5"),
      url_1_title: row.get("url_1_title"),
      url_2_title: row.get("url_2_title"),
      url_3_title: row.get("url_3_title"),
      url_4_title: row.get("url_4_title"),
      url_5_title: row.get("url_5_title"),
      intro_md: row.get("intro_md"),
      intro_html: row.get("intro_html"),
      private_key: row.get("private_key"),
      public_key: row.get("public_key"),
      ext_apub_followers_uri: row.get("ext_apub_followers_uri"),
      ext_apub_following_uri: row.get("ext_apub_following_uri"),
      ext_apub_inbox_uri: row.get("ext_apub_inbox_uri"),
      ext_apub_outbox_uri: row.get("ext_apub_outbox_uri"),
      created_at: row.get("created_at"),
    })
  }
}

impl ActivityConvertible for User {
  fn to_object(&self, _actor: &str) -> Option<Object> {
    let id = relative_to_absolute_uri(&self.fediverse_uri);
    let public_inbox_uri = format!("{}/federate/activitypub/shared-inbox", SETTINGS.server.api_fqdn);
    let inbox_uri = format!(
      "{}/federate/activitypub/inbox/{}",
      SETTINGS.server.api_fqdn, &self.handle
    );
    let outbox_uri = format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, &self.handle);
    let liked_uri = format!("{}/users/{}/likes", SETTINGS.server.api_fqdn, &self.handle);
    let followers_uri = format!("{}/users/{}/followers", SETTINGS.server.api_fqdn, &self.handle);
    let following_uri = format!("{}/users/{}/following", SETTINGS.server.api_fqdn, &self.handle);
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

    let key_props = KeyProps::builder()
      .id(Some(format!("{}#main-key", id)))
      .owner(Some(id.clone()))
      .public_key_pem(Some(self.public_key.clone()))
      .build();

    Some(
      Object::builder()
        .id(Some(id.clone()))
        .kind(Some("Person".to_string()))
        .icon(icon)
        .url(Some(Reference::Remote(id)))
        .name(Some(self.handle.clone()))
        .actors(Some(
          ActorProps::builder()
            .endpoints(Some(Reference::Map(endpoints)))
            .followers(Some(Reference::Remote(followers_uri)))
            .following(Some(Reference::Remote(following_uri)))
            .inbox(Some(Reference::Remote(inbox_uri)))
            .outbox(Some(Reference::Remote(outbox_uri)))
            .liked(Some(Reference::Remote(liked_uri)))
            .preferred_username(Some(self.handle.clone()))
            .build(),
        ))
        .key(Some(key_props))
        .build(),
    )
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;

  #[test]
  fn test_to_webfinger_returns_data() {
    let user = User {
      user_id: Uuid::from_str("ae1481a5-2eb7-4c52-93c3-e95839578dce").unwrap(),
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: "user".to_string(),
      fediverse_uri: "http://127.0.0.1:8000/api/users/user".to_string(),
      avatar_url: None,
      email: Some("user@example.com".to_string()),
      password_hash: Some("...".to_string()),
      is_external: false,
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
      private_key: "a".to_string(),
      public_key: "b".to_string(),
      ext_apub_followers_uri: None,
      ext_apub_following_uri: None,
      ext_apub_inbox_uri: None,
      ext_apub_outbox_uri: None,
      created_at: Utc::now(),
    };

    temp_env::with_vars(vec![("RUN_MODE", Some("production"))], || {
      let finger = user.to_webfinger();

      assert_eq!(&finger.subject, &user.fediverse_id);
      assert!(finger.aliases.is_some());
      assert_eq!(finger.aliases.unwrap().len(), 1);
      assert_eq!(finger.links.len(), 3);

      assert_eq!(finger.links[0].rel, "self");
      assert_eq!(finger.links[0].link_type, "application/activity+json");
      assert!(finger.links[0].href.is_some());
      assert_eq!(
        finger.links[0].href.as_ref().unwrap(),
        "http://0.0.0.0:8080/api/users/user"
      );

      assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
      assert_eq!(finger.links[1].link_type, "text/html");
      assert!(finger.links[1].href.is_some());
      assert_eq!(finger.links[1].href.as_ref().unwrap(), "http://0.0.0.0:8080/users/user");
    });
  }
}

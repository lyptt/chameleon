use std::collections::HashMap;

use sqlx::FromRow;
use uuid::Uuid;

use crate::{
  activitypub::{activity_convertible::ActivityConvertible, actor::ActorProps, object::Object, reference::Reference},
  helpers::api::relative_to_absolute_uri,
  settings::SETTINGS,
};

use super::webfinger::{WebfingerRecord, WebfingerRecordLink};

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
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
}

impl User {
  pub fn to_webfinger(&self) -> WebfingerRecord {
    WebfingerRecord {
      aliases: Some(vec![WebfingerRecordLink::build_self_uri(&self.handle)]),
      subject: self.fediverse_id.clone(),
      links: [
        WebfingerRecordLink::build_self_link(&self.handle),
        WebfingerRecordLink::build_profile_page_link(&self.handle),
        WebfingerRecordLink::build_feed_link(&self.handle),
      ]
      .into(),
    }
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

    Some(
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
            .liked(Some(Reference::Remote(liked_uri)))
            .preferred_username(Some(self.handle.clone()))
            .build(),
        ))
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

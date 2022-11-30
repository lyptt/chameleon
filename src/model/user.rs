use sqlx::FromRow;
use uuid::Uuid;

use super::webfinger::{WebfingerRecord, WebfingerRecordLink};

#[derive(Clone, FromRow)]
pub struct User {
  pub user_id: Uuid,
  pub fediverse_id: String,
  pub handle: Option<String>,
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

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;

  #[test]
  fn test_to_webfinger_returns_data() {
    let user = User {
      user_id: Uuid::from_str("ae1481a5-2eb7-4c52-93c3-e95839578dce").unwrap(),
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: Some("user".to_string()),
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

  #[test]
  fn test_to_webfinger_returns_data_for_no_handle() {
    let user = User {
      user_id: Uuid::from_str("ae1481a5-2eb7-4c52-93c3-e95839578dce").unwrap(),
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: None,
      avatar_url: None,
      email: None,
      password_hash: None,
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
        "http://0.0.0.0:8080/api/users/<unknown>"
      );

      assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
      assert_eq!(finger.links[1].link_type, "text/html");
      assert!(finger.links[1].href.is_some());
      assert_eq!(
        finger.links[1].href.as_ref().unwrap(),
        "http://0.0.0.0:8080/users/<unknown>"
      );
    });
  }
}

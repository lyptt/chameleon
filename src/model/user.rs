use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};

use super::webfinger::{WebfingerRecord, WebfingerRecordLink};

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
  user_id: i32,
  fediverse_id: String,
  handle: Option<String>,
  email: Option<String>,
  password_hash: Option<String>,
  is_external: bool,
}

impl User {
  pub async fn fetch_by_handle(handle: &String, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(pool)
      .await?;

    Ok(user)
  }

  pub async fn fetch_by_fediverse_id(fediverse_id: &String, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE fediverse_id = $1")
      .bind(fediverse_id)
      .fetch_optional(pool)
      .await?;

    Ok(user)
  }

  pub fn to_webfinger(&self) -> WebfingerRecord {
    return WebfingerRecord {
      aliases: Some(vec![WebfingerRecordLink::build_self_uri(&self.handle)]),
      subject: self.fediverse_id.clone(),
      links: [
        WebfingerRecordLink::build_self_link(&self.handle),
        WebfingerRecordLink::build_profile_page_link(&self.handle),
      ]
      .into(),
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_webfinger_returns_data() {
    let user = User {
      user_id: 1,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: Some("user".to_string()),
      email: Some("user@example.com".to_string()),
      password_hash: Some("...".to_string()),
      is_external: false,
    };

    let finger = user.to_webfinger();

    assert_eq!(&finger.subject, &user.fediverse_id);
    assert_eq!(finger.aliases.is_some(), true);
    assert_eq!(finger.aliases.unwrap().len(), 1);
    assert_eq!(finger.links.len(), 2);

    assert_eq!(finger.links[0].rel, "self");
    assert_eq!(finger.links[0].link_type, "application/activity+json");
    assert_eq!(finger.links[0].href.is_some(), true);
    assert_eq!(
      finger.links[0].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/api/users/user"
    );

    assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
    assert_eq!(finger.links[1].link_type, "text/html");
    assert_eq!(finger.links[1].href.is_some(), true);
    assert_eq!(
      finger.links[1].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/feed/user"
    );
  }

  #[test]
  fn test_to_webfinger_returns_data_for_no_handle() {
    let user = User {
      user_id: 1,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: None,
      email: None,
      password_hash: None,
      is_external: true,
    };

    let finger = user.to_webfinger();

    assert_eq!(&finger.subject, &user.fediverse_id);
    assert_eq!(finger.aliases.is_some(), true);
    assert_eq!(finger.aliases.unwrap().len(), 1);
    assert_eq!(finger.links.len(), 2);

    assert_eq!(finger.links[0].rel, "self");
    assert_eq!(finger.links[0].link_type, "application/activity+json");
    assert_eq!(finger.links[0].href.is_some(), true);
    assert_eq!(
      finger.links[0].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/api/users/<unknown>"
    );

    assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
    assert_eq!(finger.links[1].link_type, "text/html");
    assert_eq!(finger.links[1].href.is_some(), true);
    assert_eq!(
      finger.links[1].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/feed/<unknown>"
    );
  }
}

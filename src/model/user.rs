use sqlx::{Error, FromRow, Pool, Postgres};
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
}

impl User {
  pub async fn fetch_by_handle(handle: &String, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(pool)
      .await?;

    Ok(user)
  }

  pub async fn fetch_id_by_handle(handle: &String, pool: &Pool<Postgres>) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(pool)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  pub async fn fetch_id_by_fediverse_id(fediverse_id: &String, pool: &Pool<Postgres>) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM users WHERE fediverse_id = $1")
      .bind(fediverse_id)
      .fetch_optional(pool)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  pub async fn fetch_by_fediverse_id(fediverse_id: &String, pool: &Pool<Postgres>) -> Result<Option<User>, Error> {
    let user = sqlx::query_as("SELECT * FROM users WHERE fediverse_id = $1")
      .bind(fediverse_id)
      .fetch_optional(pool)
      .await?;

    Ok(user)
  }

  pub async fn fetch_password_hash(handle: &str, pool: &Pool<Postgres>) -> Result<Option<String>, Error> {
    let password_hash = sqlx::query_scalar("SELECT password_hash FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(pool)
      .await?;

    Ok(password_hash)
  }

  pub fn to_webfinger(&self) -> WebfingerRecord {
    WebfingerRecord {
      aliases: Some(vec![WebfingerRecordLink::build_self_uri(&self.fediverse_id)]),
      subject: self.fediverse_id.clone(),
      links: [
        WebfingerRecordLink::build_self_link(&self.fediverse_id),
        WebfingerRecordLink::build_profile_page_link(&self.fediverse_id),
        WebfingerRecordLink::build_feed_link(&self.fediverse_id),
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
    };

    let finger = user.to_webfinger();

    assert_eq!(&finger.subject, &user.fediverse_id);
    assert!(finger.aliases.is_some());
    assert_eq!(finger.aliases.unwrap().len(), 1);
    assert_eq!(finger.links.len(), 2);

    assert_eq!(finger.links[0].rel, "self");
    assert_eq!(finger.links[0].link_type, "application/activity+json");
    assert!(finger.links[0].href.is_some());
    assert_eq!(
      finger.links[0].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/api/users/user"
    );

    assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
    assert_eq!(finger.links[1].link_type, "text/html");
    assert!(finger.links[1].href.is_some());
    assert_eq!(
      finger.links[1].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/feed/user"
    );
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
    };

    let finger = user.to_webfinger();

    assert_eq!(&finger.subject, &user.fediverse_id);
    assert!(finger.aliases.is_some());
    assert_eq!(finger.aliases.unwrap().len(), 1);
    assert_eq!(finger.links.len(), 2);

    assert_eq!(finger.links[0].rel, "self");
    assert_eq!(finger.links[0].link_type, "application/activity+json");
    assert!(finger.links[0].href.is_some());
    assert_eq!(
      finger.links[0].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/api/users/<unknown>"
    );

    assert_eq!(finger.links[1].rel, "http://webfinger.net/rel/profile-page");
    assert_eq!(finger.links[1].link_type, "text/html");
    assert!(finger.links[1].href.is_some());
    assert_eq!(
      finger.links[1].href.as_ref().unwrap(),
      "http://127.0.0.1:8000/feed/<unknown>"
    );
  }
}

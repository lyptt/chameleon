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
  pub handle: Option<String>,
  pub avatar_url: Option<String>,
  pub email: Option<String>,
}

impl From<User> for UserAccountPub {
  fn from(u: User) -> Self {
    UserAccountPub {
      user_id: u.user_id,
      fediverse_id: u.fediverse_id,
      handle: u.handle,
      avatar_url: u.avatar_url,
      email: u.email,
    }
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
    };
    let user_cmp = user.clone();

    let val: UserAccountPub = user.into();

    assert_eq!(val.user_id, user_cmp.user_id);
    assert_eq!(val.fediverse_id, user_cmp.fediverse_id);
    assert_eq!(val.handle, user_cmp.handle);
    assert_eq!(val.email, user_cmp.email);
  }
}

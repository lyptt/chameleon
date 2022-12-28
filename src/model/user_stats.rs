use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::db::FromRow;

#[derive(Deserialize, Serialize)]
pub struct UserStats {
  pub following_count: i64,
  pub followers_count: i64,
  pub following_user: bool,
  pub user_is_you: bool,
}

impl FromRow for UserStats {
  fn from_row(row: Row) -> Option<Self> {
    Some(UserStats {
      following_count: row.get("following_count"),
      followers_count: row.get("followers_count"),
      following_user: row.get("following_user"),
      user_is_you: row.get("user_is_you"),
    })
  }
}

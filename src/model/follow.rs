use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize)]
/// Represents a user's follow on another user
pub struct Follow {
  pub follower_id: Uuid,
  pub user_id: Uuid,
  pub following_user_id: Uuid,
  pub created_at: DateTime<Utc>,
}

impl FromRow for Follow {
  fn from_row(row: Row) -> Option<Self> {
    Some(Follow {
      follower_id: row.get("follower_id"),
      user_id: row.get("user_id"),
      following_user_id: row.get("following_user_id"),
      created_at: row.get("created_at"),
    })
  }
}

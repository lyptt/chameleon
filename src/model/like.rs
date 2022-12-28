use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize)]
/// Represents a user's like on a post
pub struct Like {
  pub like_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub created_at: DateTime<Utc>,
}

impl FromRow for Like {
  fn from_row(row: Row) -> Option<Self> {
    Some(Like {
      like_id: row.get("like_id"),
      user_id: row.get("user_id"),
      post_id: row.get("post_id"),
      created_at: row.get("created_at"),
    })
  }
}

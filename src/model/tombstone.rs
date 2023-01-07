use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize)]
/// Represents a user's tombstone on another user
pub struct Tombstone {
  pub tombstone_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub deleted_at: DateTime<Utc>,
  pub fediverse_uri: String,
  pub former_type: String,
}

impl FromRow for Tombstone {
  fn from_row(row: Row) -> Option<Self> {
    Some(Tombstone {
      tombstone_id: row.get("tombstone_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      deleted_at: row.get("deleted_at"),
      fediverse_uri: row.get("fediverse_uri"),
      former_type: row.get("former_type"),
    })
  }
}

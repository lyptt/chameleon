use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OrbitModerator {
  pub orbit_moderator_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub is_owner: bool,
  pub orbit_id: Uuid,
  pub user_id: Uuid,
}

impl FromRow for OrbitModerator {
  fn from_row(row: Row) -> Option<Self> {
    Some(OrbitModerator {
      orbit_moderator_id: row.get("orbit_moderator_id"),
      created_at: row.get("created_at"),
      is_owner: row.get("is_owner"),
      orbit_id: row.get("orbit_id"),
      user_id: row.get("user_id"),
    })
  }
}

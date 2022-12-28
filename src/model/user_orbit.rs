use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct UserOrbit {
  pub user_orbit_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub orbit_id: Uuid,
  pub user_id: Uuid,
}

impl FromRow for UserOrbit {
  fn from_row(row: Row) -> Option<Self> {
    Some(UserOrbit {
      user_orbit_id: row.get("user_orbit_id"),
      created_at: row.get("created_at"),
      orbit_id: row.get("orbit_id"),
      user_id: row.get("user_id"),
    })
  }
}

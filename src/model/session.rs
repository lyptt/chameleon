use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize)]
pub struct Session {
  pub session_id: Uuid,
  pub user_id: Uuid,
  pub app_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub access_expires_at: DateTime<Utc>,
  pub refresh_expires_at: DateTime<Utc>,
}

impl FromRow for Session {
  fn from_row(row: Row) -> Option<Self> {
    Some(Session {
      session_id: row.get("session_id"),
      user_id: row.get("user_id"),
      app_id: row.get("app_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      access_expires_at: row.get("access_expires_at"),
      refresh_expires_at: row.get("refresh_expires_at"),
    })
  }
}

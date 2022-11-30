use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Session {
  pub session_id: Uuid,
  pub user_id: Uuid,
  pub app_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub access_expires_at: DateTime<Utc>,
  pub refresh_expires_at: DateTime<Utc>,
}

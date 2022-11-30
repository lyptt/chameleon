use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's follow on another user
pub struct Follow {
  pub follower_id: Uuid,
  pub user_id: Uuid,
  pub following_user_id: Uuid,
  pub created_at: DateTime<Utc>,
}

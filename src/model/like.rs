use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
/// Represents a user's like on a post
pub struct Like {
  pub like_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub created_at: DateTime<Utc>,
}

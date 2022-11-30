use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's comment on a post
pub struct Comment {
  pub comment_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub content_md: String,
  pub content_html: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

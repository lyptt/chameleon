use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{access_type::AccessType, event_type::EventType};

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's follow on another user
pub struct Event {
  pub event_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub source_user_id: Uuid,
  pub target_user_id: Option<Uuid>,
  pub visibility: AccessType,
  pub post_id: Option<Uuid>,
  pub like_id: Option<Uuid>,
  pub comment_id: Option<Uuid>,
  pub event_type: EventType,
}

pub struct NewEvent {
  pub source_user_id: Uuid,
  pub target_user_id: Option<Uuid>,
  pub visibility: AccessType,
  pub post_id: Option<Uuid>,
  pub like_id: Option<Uuid>,
  pub comment_id: Option<Uuid>,
  pub event_type: EventType,
}

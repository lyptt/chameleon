use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

use super::{access_type::AccessType, event_type::EventType};

#[derive(Deserialize, Serialize)]
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

impl FromRow for Event {
  fn from_row(row: Row) -> Option<Self> {
    Some(Event {
      event_id: row.get("event_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      source_user_id: row.get("source_user_id"),
      target_user_id: row.get("target_user_id"),
      visibility: AccessType::from_str(row.get("visibility")).unwrap_or_default(),
      post_id: row.get("post_id"),
      like_id: row.get("like_id"),
      comment_id: row.get("comment_id"),
      event_type: EventType::from_str(row.get("event_type")).unwrap_or_default(),
    })
  }
}

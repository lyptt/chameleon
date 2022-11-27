use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
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

impl Event {
  pub async fn create_event(event: NewEvent, pool: &Pool<Postgres>) -> Result<(), Error> {
    let event_id = Uuid::new_v4();

    sqlx::query(
      "INSERT INTO events (event_id, source_user_id, target_user_id, visibility, post_id, like_id, comment_id, event_type) VALUES($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(event_id)
    .bind(event.source_user_id)
    .bind(event.target_user_id)
    .bind(event.visibility.to_string())
    .bind(event.post_id)
    .bind(event.like_id)
    .bind(event.comment_id)
    .bind(event.event_type.to_string())
    .execute(pool)
    .await?;

    Ok(())
  }

  pub async fn update_event(&self, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query(
      "UPDATE events SET source_user_id = $2, target_user_id = $3, visibility = $4, post_id = $5, like_id = $6, comment_id = $7, event_type = $8 WHERE event_id = $1",
    )
    .bind(self.event_id)
    .bind(self.source_user_id)
    .bind(self.target_user_id)
    .bind(self.visibility.to_string())
    .bind(self.post_id)
    .bind(self.like_id)
    .bind(self.comment_id)
    .bind(self.event_type.to_string())
    .execute(pool)
    .await?;

    Ok(())
  }

  pub async fn delete_event(event_id: &Uuid, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("DELETE FROM events WHERE event_id = $1")
      .bind(event_id)
      .execute(pool)
      .await?;

    Ok(())
  }
}

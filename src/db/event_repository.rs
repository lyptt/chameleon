use std::sync::Arc;

use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    event::{Event, NewEvent},
    event_type::EventType,
  },
};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait EventRepo {
  async fn create_event(&self, event: NewEvent) -> Result<(), LogicErr>;
  async fn update_event(&self, event: &Event) -> Result<(), LogicErr>;
  async fn delete_event(&self, event_id: &Uuid) -> Result<(), LogicErr>;
  async fn delete_post_events(&self, post_id: &Uuid, user_id: &Uuid, event_type: EventType) -> Result<(), LogicErr>;
}

pub type EventPool = Arc<dyn EventRepo + Send + Sync>;

pub struct DbEventRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl EventRepo for DbEventRepo {
  async fn create_event(&self, event: NewEvent) -> Result<(), LogicErr> {
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
    .execute(&self.db)
    .await.map_err(map_db_err)?;

    Ok(())
  }

  async fn update_event(&self, event: &Event) -> Result<(), LogicErr> {
    sqlx::query(
      "UPDATE events SET source_user_id = $2, target_user_id = $3, visibility = $4, post_id = $5, like_id = $6, comment_id = $7, event_type = $8 WHERE event_id = $1",
    )
    .bind(event.event_id)
    .bind(event.source_user_id)
    .bind(event.target_user_id)
    .bind(event.visibility.to_string())
    .bind(event.post_id)
    .bind(event.like_id)
    .bind(event.comment_id)
    .bind(event.event_type.to_string())
    .execute(&self.db)
    .await.map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_event(&self, event_id: &Uuid) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM events WHERE event_id = $1")
      .bind(event_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_post_events(&self, post_id: &Uuid, user_id: &Uuid, event_type: EventType) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM events WHERE post_id = $1 AND source_user_id = $2 AND event_type = $3")
      .bind(post_id)
      .bind(user_id)
      .bind(event_type.to_string())
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }
}

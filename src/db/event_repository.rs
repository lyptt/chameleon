use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    event::{Event, NewEvent},
    event_type::EventType,
  },
};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
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
  pub db: Pool,
}

#[async_trait]
impl EventRepo for DbEventRepo {
  async fn create_event(&self, event: NewEvent) -> Result<(), LogicErr> {
    let event_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      r#"INSERT INTO events (event_id, source_user_id, target_user_id, visibility, post_id, like_id, comment_id, event_type)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
      &[&event_id,
      &event.source_user_id,
      &event.target_user_id,
      &event.visibility.to_string(),
      &event.post_id,
      &event.like_id,
      &event.comment_id,
      &event.event_type.to_string(),],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn update_event(&self, event: &Event) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      r#"UPDATE events SET source_user_id = $2, target_user_id = $3, visibility = $4, post_id = $5, like_id = $6,
      comment_id = $7, event_type = $8 WHERE event_id = $1"#,
      &[
        &event.event_id,
        &event.source_user_id,
        &event.target_user_id,
        &event.visibility.to_string(),
        &event.post_id,
        &event.like_id,
        &event.comment_id,
        &event.event_type.to_string(),
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_event(&self, event_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute("DELETE FROM events WHERE event_id = $1", &[&event_id])
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_post_events(&self, post_id: &Uuid, user_id: &Uuid, event_type: EventType) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM events WHERE post_id = $1 AND source_user_id = $2 AND event_type = $3",
      &[&post_id, &user_id, &event_type.to_string()],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }
}

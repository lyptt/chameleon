use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{helpers::api::map_db_err, logic::LogicErr};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LikeRepo {
  async fn create_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<Uuid, LogicErr>;
  async fn delete_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr>;
}

pub type LikePool = Arc<dyn LikeRepo + Send + Sync>;

pub struct DbLikeRepo {
  pub db: Pool,
}

#[async_trait]
impl LikeRepo for DbLikeRepo {
  async fn create_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<Uuid, LogicErr> {
    let event_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "INSERT INTO likes (like_id, user_id, post_id) VALUES ($1, $2, $3) RETURNING like_id",
        &[&event_id, &user_id, &post_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn delete_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM likes WHERE post_id = $1 AND user_id = $2",
      &[&post_id, &user_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }
}

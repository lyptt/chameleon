use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::{helpers::api::map_db_err, logic::LogicErr};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait LikeRepo {
  async fn create_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<Uuid, LogicErr>;
  async fn delete_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr>;
}

pub type LikePool = Arc<dyn LikeRepo + Send + Sync>;

pub struct DbLikeRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl LikeRepo for DbLikeRepo {
  async fn create_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<Uuid, LogicErr> {
    let like_id = Uuid::new_v4();

    let id = sqlx::query_scalar("INSERT INTO likes (like_id, user_id, post_id) VALUES ($1, $2, $3) RETURNING like_id")
      .bind(like_id)
      .bind(user_id)
      .bind(post_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(id)
  }

  async fn delete_like(&self, user_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM likes WHERE post_id = $1 AND user_id = $2")
      .bind(post_id)
      .bind(user_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }
}

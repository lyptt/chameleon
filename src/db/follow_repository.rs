use crate::{helpers::api::map_db_err, logic::LogicErr, model::follow::Follow};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use super::FromRow;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait FollowRepo {
  async fn create_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<Uuid, LogicErr>;
  async fn delete_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<(), LogicErr>;
  async fn user_follows_poster(&self, post_id: &Uuid, user_id: &Uuid) -> bool;
  async fn user_follows_user(&self, following_user_id: &Uuid, followed_user_id: &Uuid) -> bool;
  async fn fetch_user_followers(&self, user_id: &Uuid) -> Option<Vec<Follow>>;
}

pub type FollowPool = Arc<dyn FollowRepo + Send + Sync>;

pub struct DbFollowRepo {
  pub db: Pool,
}

#[async_trait]
impl FollowRepo for DbFollowRepo {
  async fn create_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<Uuid, LogicErr> {
    let follower_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "INSERT INTO followers (follower_id, user_id, following_user_id) VALUES ($1, $2, $3) RETURNING follower_id",
        &[&follower_id, &user_id, &following_user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn delete_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM followers WHERE following_user_id = $1 AND user_id = $2",
      &[&following_user_id, &user_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  /// Fetches a boolean indicator of if the specified user follows the user that created the specified post
  async fn user_follows_poster(&self, post_id: &Uuid, user_id: &Uuid) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };

    let row = match db
      .query_one(
        r#"SELECT count(f.*) >= 1 AS following FROM followers f
      INNER JOIN posts p
      ON p.user_id = f.following_user_id
      WHERE p.post_id = $1
      AND f.user_id = $2"#,
        &[&post_id, &user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
  }

  /// Fetches a boolean indicator of if the source user follows the target user
  async fn user_follows_user(&self, following_user_id: &Uuid, followed_user_id: &Uuid) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };

    let row = match db
      .query_one(
        r#"SELECT count(*) >= 1 AS following FROM followers
        WHERE user_id = $1
        AND following_user_id = $2"#,
        &[&following_user_id, &followed_user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
  }

  async fn fetch_user_followers(&self, user_id: &Uuid) -> Option<Vec<Follow>> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let rows = match db
      .query(
        "SELECT * FROM followers WHERE following_user_id = $1 AND user_id != following_user_id",
        &[&user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(rows) => rows,
      Err(_) => return None,
    };

    Some(rows.into_iter().flat_map(Follow::from_row).collect())
  }
}

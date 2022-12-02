use std::sync::Arc;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::follow::Follow};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
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
  pub db: Pool<Postgres>,
}

#[async_trait]
impl FollowRepo for DbFollowRepo {
  async fn create_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<Uuid, LogicErr> {
    let follower_id = Uuid::new_v4();

    let id = sqlx::query_scalar(
      "INSERT INTO followers (follower_id, user_id, following_user_id) VALUES ($1, $2, $3) RETURNING follower_id",
    )
    .bind(follower_id)
    .bind(user_id)
    .bind(following_user_id)
    .fetch_one(&self.db)
    .await
    .map_err(map_db_err)?;

    Ok(id)
  }

  async fn delete_follow(&self, user_id: &Uuid, following_user_id: &Uuid) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM followers WHERE following_user_id = $1 AND user_id = $2")
      .bind(following_user_id)
      .bind(user_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  /// Fetches a boolean indicator of if the specified user follows the user that created the specified post
  async fn user_follows_poster(&self, post_id: &Uuid, user_id: &Uuid) -> bool {
    sqlx::query_scalar(
      "SELECT count(f.*) >= 1 AS following FROM followers f
        INNER JOIN posts p
        ON p.user_id = f.following_user_id
        WHERE p.post_id = $1
        AND f.user_id = $2",
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_one(&self.db)
    .await
    .unwrap_or(false)
  }

  /// Fetches a boolean indicator of if the source user follows the target user
  async fn user_follows_user(&self, following_user_id: &Uuid, followed_user_id: &Uuid) -> bool {
    sqlx::query_scalar(
      "SELECT count(*) >= 1 AS following FROM followers
        WHERE user_id = $1
        AND following_user_id = $2",
    )
    .bind(following_user_id)
    .bind(followed_user_id)
    .fetch_one(&self.db)
    .await
    .unwrap_or(false)
  }

  async fn fetch_user_followers(&self, user_id: &Uuid) -> Option<Vec<Follow>> {
    let result =
      sqlx::query_as("SELECT * FROM followers WHERE following_user_id = $1 AND user_id != following_user_id")
        .bind(user_id)
        .fetch_all(&self.db)
        .await;

    match result {
      Ok(follows) => Some(follows),
      Err(_) => None,
    }
  }
}

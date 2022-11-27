use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's follow on another user
pub struct Follow {
  pub follower_id: Uuid,
  pub user_id: Uuid,
  pub following_user_id: Uuid,
  pub created_at: DateTime<Utc>,
}

impl Follow {
  pub async fn create_follow(user_id: &Uuid, following_user_id: &Uuid, pool: &Pool<Postgres>) -> Result<Uuid, Error> {
    let follower_id = Uuid::new_v4();

    let id = sqlx::query_scalar(
      "INSERT INTO followers (follower_id, user_id, following_user_id) VALUES ($1, $2, $3) RETURNING follower_id",
    )
    .bind(follower_id)
    .bind(user_id)
    .bind(following_user_id)
    .fetch_one(pool)
    .await?;

    Ok(id)
  }

  pub async fn delete_follow(user_id: &Uuid, following_user_id: &Uuid, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("DELETE FROM followers WHERE following_user_id = $1 AND user_id = $2")
      .bind(following_user_id)
      .bind(user_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  /// Fetches a boolean indicator of if the specified user follows the user that created the specified post
  pub async fn user_follows_poster(post_id: &Uuid, user_id: &Uuid, pool: &Pool<Postgres>) -> bool {
    sqlx::query_scalar(
      "SELECT count(f.*) >= 1 AS following FROM followers f
        INNER JOIN posts p
        ON p.user_id = f.following_user_id
        WHERE p.post_id = $1
        AND f.user_id = $2",
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
    .unwrap_or(false)
  }

  /// Fetches a boolean indicator of if the source user follows the target user
  pub async fn user_follows_user(following_user_id: &Uuid, followed_user_id: &Uuid, pool: &Pool<Postgres>) -> bool {
    sqlx::query_scalar(
      "SELECT count(*) >= 1 AS following FROM followers
        WHERE user_id = $1
        AND following_user_id = $2",
    )
    .bind(following_user_id)
    .bind(followed_user_id)
    .fetch_one(pool)
    .await
    .unwrap_or(false)
  }

  pub async fn fetch_user_followers(user_id: &Uuid, pool: &Pool<Postgres>) -> Option<Vec<Follow>> {
    let result =
      sqlx::query_as("SELECT * FROM followers WHERE following_user_id = $1 AND user_id != following_user_id")
        .bind(user_id)
        .fetch_all(pool)
        .await;

    match result {
      Ok(follows) => Some(follows),
      Err(_) => None,
    }
  }
}

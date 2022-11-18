use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
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
}

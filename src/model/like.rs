use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
/// Represents a user's like on a post
pub struct Like {
  pub like_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub created_at: DateTime<Utc>,
}

impl Like {
  pub async fn create_like(user_id: &Uuid, post_id: &Uuid, pool: &Pool<Postgres>) -> Result<Uuid, Error> {
    let like_id = Uuid::new_v4();

    let id = sqlx::query_scalar("INSERT INTO likes (like_id, user_id, post_id) VALUES ($1, $2, $3) RETURNING like_id")
      .bind(like_id)
      .bind(user_id)
      .bind(post_id)
      .fetch_one(pool)
      .await?;

    Ok(id)
  }

  pub async fn delete_like(user_id: &Uuid, post_id: &Uuid, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("DELETE FROM likes WHERE post_id = $1 AND user_id = $2")
      .bind(post_id)
      .bind(user_id)
      .execute(pool)
      .await?;

    Ok(())
  }
}

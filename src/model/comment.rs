use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's comment on a post
pub struct Comment {
  pub comment_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub content_md: String,
  pub content_html: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Comment {
  pub async fn create_comment(
    user_id: &Uuid,
    post_id: &Uuid,
    content_md: &String,
    content_html: &String,
    pool: &Pool<Postgres>,
  ) -> Result<Uuid, Error> {
    let comment_id = Uuid::new_v4();

    let id = sqlx::query_scalar(
      "INSERT INTO comments (comment_id, user_id, post_id, content_md, content_html) VALUES ($1, $2, $3, $4, $5) RETURNING comment_id",
    )
    .bind(comment_id)
    .bind(user_id)
    .bind(post_id)
    .bind(content_md)
    .bind(content_html)
    .fetch_one(pool)
    .await?;

    Ok(id)
  }

  pub async fn delete_comment(
    user_id: &Uuid,
    post_id: &Uuid,
    comment_id: &Uuid,
    pool: &Pool<Postgres>,
  ) -> Result<(), Error> {
    sqlx::query("DELETE FROM comments WHERE post_id = $1 AND user_id = $2 AND comment_id = $3")
      .bind(post_id)
      .bind(user_id)
      .bind(comment_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn fetch_comments_count(
    post_id: &Uuid,
    own_user_id: &Option<Uuid>,
    pool: &Pool<Postgres>,
  ) -> Result<i64, Error> {
    sqlx::query_scalar(include_str!("../db/fetch_post_comments_count.sql"))
      .bind(own_user_id)
      .bind(post_id)
      .fetch_one(pool)
      .await
  }
}

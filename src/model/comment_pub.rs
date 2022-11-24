use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, ToSchema)]
/// Represents a user's comment on a post
pub struct CommentPub {
  pub comment_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub content_md: String,
  pub content_html: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub user_handle: String,
  pub user_fediverse_id: String,
  pub user_avatar_url: Option<String>,
  pub likes: i64,
  pub liked: Option<bool>,
}

impl CommentPub {
  pub async fn fetch_comments(
    post_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<CommentPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_post_comments.sql"))
      .bind(own_user_id)
      .bind(post_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(pool)
      .await?;

    Ok(post)
  }
}

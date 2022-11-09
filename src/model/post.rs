use super::access_type::AccessType;

use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Post {
  post_id: Uuid,
  user_id: Uuid,
  uri: String,
  is_external: bool,
  content_md: String,
  content_html: String,
  content_image_url_small: Option<String>,
  content_image_url_medium: Option<String>,
  content_image_url_large: Option<String>,
  content_image_storage_ref: String,
  visibility: AccessType,
  created_at: chrono::DateTime<chrono::Utc>,
  updated_at: chrono::DateTime<chrono::Utc>,
  deletion_scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Post {
  pub async fn fetch_by_user(
    user_id: &str,
    visibilities: &Vec<AccessType>,
    limit: i32,
    skip: i32,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<Post>, Error> {
    // There's no way yet to bind a Vec<T> in sqlx, so we need to use the limited available postgres support by
    // binding a slice of a built-in supported type i.e. Vec<String>.
    let visibilities_strs: Vec<String> = visibilities.iter().map(|item| item.to_string()).collect();

    let post = sqlx::query_as(
      "SELECT * FROM posts WHERE handle = $1 AND visibility = ANY($2) LIMIT $3 SKIP $4 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .bind(&visibilities_strs[..])
    .bind(limit)
    .bind(skip)
    .fetch_all(pool)
    .await?;

    Ok(post)
  }

  pub async fn count_by_user(
    user_id: &str,
    visibilities: &Vec<AccessType>,
    pool: &Pool<Postgres>,
  ) -> Result<i64, Error> {
    // There's no way yet to bind a Vec<T> in sqlx, so we need to use the limited available postgres support by
    // binding a slice of a built-in supported type i.e. Vec<String>.
    let visibilities_strs: Vec<String> = visibilities.iter().map(|item| item.to_string()).collect();
    let count = sqlx::query_scalar("SELECT COUNT(p.*) FROM posts p INNER JOIN users u ON p.user_id = u.user_id WHERE u.handle = $1 AND visibility = ANY($2)")
      .bind(user_id)
      .bind(&visibilities_strs[..])
      .fetch_one(pool)
      .await?;

    Ok(count)
  }
}

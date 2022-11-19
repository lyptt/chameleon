use super::access_type::AccessType;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Post {
  pub post_id: Uuid,
  pub user_id: Uuid,
  pub uri: String,
  pub is_external: bool,
  pub content_md: String,
  pub content_html: String,
  pub content_image_uri_small: Option<String>,
  pub content_image_uri_medium: Option<String>,
  pub content_image_uri_large: Option<String>,
  pub content_width_small: Option<i32>,
  pub content_width_medium: Option<i32>,
  pub content_width_large: Option<i32>,
  pub content_height_small: Option<i32>,
  pub content_height_medium: Option<i32>,
  pub content_height_large: Option<i32>,
  pub content_type_small: Option<String>,
  pub content_type_medium: Option<String>,
  pub content_type_large: Option<String>,
  pub content_image_storage_ref: Option<String>,
  pub content_blurhash: Option<String>,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub deletion_scheduled_at: Option<DateTime<Utc>>,
}

impl Post {
  pub async fn create_post(
    user_id: &Uuid,
    content_md: &String,
    content_html: &String,
    visibility: &AccessType,
    pool: &Pool<Postgres>,
  ) -> Result<Uuid, Error> {
    let post_id = Uuid::new_v4();
    let uri = post_id.to_string();

    let id = sqlx::query_scalar(
      "INSERT INTO posts (post_id, user_id, content_md, content_html, visibility, uri) VALUES ($1, $2, $3, $4, $5, $6) RETURNING post_id",
    )
    .bind(post_id)
    .bind(user_id)
    .bind(content_md)
    .bind(content_html)
    .bind(visibility.to_string())
    .bind(uri)
    .fetch_one(pool)
    .await?;

    Ok(id)
  }

  pub async fn update_post_content_storage(
    post_id: &Uuid,
    content_image_storage_ref: &String,
    pool: &Pool<Postgres>,
  ) -> Result<(), Error> {
    sqlx::query("UPDATE posts SET content_image_storage_ref = $1 WHERE post_id = $2")
      .bind(content_image_storage_ref)
      .bind(post_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn user_owns_post(user_id: &Uuid, post_id: &Uuid, pool: &Pool<Postgres>) -> bool {
    let result: Result<i64, Error> =
      sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = $1 AND post_id = $2")
        .bind(user_id)
        .bind(post_id)
        .fetch_one(pool)
        .await;

    match result {
      Ok(count) => count > 0,
      Err(_) => false,
    }
  }

  pub async fn find_optional_by_id(post_id: &Uuid, pool: &Pool<Postgres>) -> Option<Post> {
    let result = sqlx::query_as("SELECT * FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(pool)
      .await;

    match result {
      Ok(post) => post,
      Err(_) => None,
    }
  }

  pub async fn update_post_content(&self, pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::query("UPDATE posts SET content_type_large = $1, content_type_medium = $2, content_type_small = $3, content_width_large = $4, 
    content_height_large = $5, content_width_medium = $6, content_height_medium = $7, content_width_small = $8,
     content_height_small = $9, content_image_uri_large = $10, content_image_uri_medium = $11, content_image_uri_small = $12, content_blurhash = $13 
     WHERE post_id = $14")
      .bind(&self.content_type_large)
      .bind(&self.content_type_medium)
      .bind(&self.content_type_small)
      .bind(self.content_width_large)
      .bind(self.content_height_large)
      .bind(self.content_width_medium)
      .bind(self.content_height_medium)
      .bind(self.content_width_small)
      .bind(self.content_height_small)
      .bind(&self.content_image_uri_large)
      .bind(&self.content_image_uri_medium)
      .bind(&self.content_image_uri_small)
      .bind(&self.content_blurhash)
      .bind(self.post_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn fetch_visibility_by_id(post_id: &Uuid, pool: &Pool<Postgres>) -> Option<AccessType> {
    match sqlx::query_scalar("SELECT visibility FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(pool)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  pub async fn fetch_owner_by_id(post_id: &Uuid, pool: &Pool<Postgres>) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(pool)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }
}

use std::sync::Arc;

use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{access_type::AccessType, post::Post, post_event::PostEvent},
};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepo {
  async fn fetch_user_own_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of the posts in the user's feed from their own perspective, i.e. all of the posts they have submitted
  async fn count_user_own_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  /// Fetches the user's federated feed, i.e. what users on any server can see
  async fn fetch_user_federated_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of the user's posts in their federated feed, i.e.
  /// what users on any server can see
  async fn count_user_federated_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  /// Fetches the user's public feed, i.e. what users that follow this user
  /// can see, or alternatively all the user's public posts
  async fn fetch_user_public_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of posts in the user's public feed, i.e. what users that follow this
  /// user can see, or alternatively all the user's public posts
  async fn count_user_public_feed(&self, target_user_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr>;
  /// Fetches the global federated feed, i.e. what users not signed into this instance can see
  async fn fetch_global_federated_feed(&self, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the post count for the global federated feed, i.e. what users not signed into this instance can see
  async fn count_global_federated_feed(&self) -> Result<i64, LogicErr>;
  /// Fetches the specified post from a user's own perspective
  async fn fetch_post(&self, post_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr>;
  async fn create_post(
    &self,
    user_id: &Uuid,
    content_md: &str,
    content_html: &str,
    visibility: &AccessType,
  ) -> Result<Uuid, LogicErr>;
  async fn update_post_content_storage(&self, post_id: &Uuid, content_image_storage_ref: &str) -> Result<(), LogicErr>;
  async fn user_owns_post(&self, user_id: &Uuid, post_id: &Uuid) -> bool;
  async fn find_optional_by_id(&self, post_id: &Uuid) -> Option<Post>;
  async fn update_post_content(&self, post: &Post) -> Result<(), LogicErr>;
  async fn fetch_visibility_by_id(&self, post_id: &Uuid) -> Option<AccessType>;
  async fn fetch_owner_by_id(&self, post_id: &Uuid) -> Option<Uuid>;
  async fn fetch_post_count(&self) -> i64;
  /// Fetches the user's public feed, i.e. what users that follow this user
  /// can see, or alternatively all the user's public posts
  async fn fetch_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of posts in the user's public feed, i.e. what users that follow this
  /// user can see, or alternatively all the user's public posts
  async fn count_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
  ) -> Result<i64, LogicErr>;
}

pub type PostPool = Arc<dyn PostRepo + Send + Sync>;

pub struct DbPostRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl PostRepo for DbPostRepo {
  async fn fetch_user_own_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_user_own_feed.sql"))
      .bind(user_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn count_user_own_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr> {
    let count = sqlx::query_scalar(include_str!("./sql/count_user_own_feed.sql"))
      .bind(user_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(count)
  }

  async fn fetch_user_federated_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_user_federated_feed.sql"))
      .bind(user_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn count_user_federated_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr> {
    let count = sqlx::query_scalar(include_str!("./sql/count_user_federated_feed.sql"))
      .bind(user_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(count)
  }

  async fn fetch_user_public_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_user_public_feed.sql"))
      .bind(target_user_id)
      .bind(own_user_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn count_user_public_feed(&self, target_user_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr> {
    let count = sqlx::query_scalar(include_str!("./sql/count_user_public_feed.sql"))
      .bind(target_user_id)
      .bind(own_user_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(count)
  }

  async fn fetch_global_federated_feed(&self, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_global_federated_feed.sql"))
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn count_global_federated_feed(&self) -> Result<i64, LogicErr> {
    let count = sqlx::query_scalar(include_str!("./sql/count_global_federated_feed.sql"))
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(count)
  }

  async fn fetch_post(&self, post_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_post.sql"))
      .bind(post_id)
      .bind(user_id)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn create_post(
    &self,
    user_id: &Uuid,
    content_md: &str,
    content_html: &str,
    visibility: &AccessType,
  ) -> Result<Uuid, LogicErr> {
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
    .fetch_one(&self.db)
    .await.map_err(map_db_err)?;

    Ok(id)
  }

  async fn update_post_content_storage(&self, post_id: &Uuid, content_image_storage_ref: &str) -> Result<(), LogicErr> {
    sqlx::query("UPDATE posts SET content_image_storage_ref = $1 WHERE post_id = $2")
      .bind(content_image_storage_ref)
      .bind(post_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn user_owns_post(&self, user_id: &Uuid, post_id: &Uuid) -> bool {
    let result: Result<i64, LogicErr> =
      sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE user_id = $1 AND post_id = $2")
        .bind(user_id)
        .bind(post_id)
        .fetch_one(&self.db)
        .await
        .map_err(map_db_err);

    match result {
      Ok(count) => count > 0,
      Err(_) => false,
    }
  }

  async fn find_optional_by_id(&self, post_id: &Uuid) -> Option<Post> {
    let result = sqlx::query_as("SELECT * FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(&self.db)
      .await;

    match result {
      Ok(post) => post,
      Err(_) => None,
    }
  }

  async fn update_post_content(&self, post: &Post) -> Result<(), LogicErr> {
    sqlx::query("UPDATE posts SET content_type_large = $1, content_type_medium = $2, content_type_small = $3, content_width_large = $4, 
    content_height_large = $5, content_width_medium = $6, content_height_medium = $7, content_width_small = $8,
     content_height_small = $9, content_image_uri_large = $10, content_image_uri_medium = $11, content_image_uri_small = $12, content_blurhash = $13 
     WHERE post_id = $14")
      .bind(&post.content_type_large)
      .bind(&post.content_type_medium)
      .bind(&post.content_type_small)
      .bind(post.content_width_large)
      .bind(post.content_height_large)
      .bind(post.content_width_medium)
      .bind(post.content_height_medium)
      .bind(post.content_width_small)
      .bind(post.content_height_small)
      .bind(&post.content_image_uri_large)
      .bind(&post.content_image_uri_medium)
      .bind(&post.content_image_uri_small)
      .bind(&post.content_blurhash)
      .bind(post.post_id)
      .execute(&self.db)
      .await.map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_visibility_by_id(&self, post_id: &Uuid) -> Option<AccessType> {
    match sqlx::query_scalar("SELECT visibility FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(&self.db)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  async fn fetch_owner_by_id(&self, post_id: &Uuid) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM posts WHERE post_id = $1")
      .bind(post_id)
      .fetch_optional(&self.db)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  async fn fetch_post_count(&self) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM posts")
      .fetch_one(&self.db)
      .await
      .unwrap_or(0)
  }

  async fn fetch_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_user_public_likes_feed.sql"))
      .bind(target_user_id)
      .bind(own_user_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn count_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
  ) -> Result<i64, LogicErr> {
    let count = sqlx::query_scalar(include_str!("./sql/count_user_public_likes_feed.sql"))
      .bind(target_user_id)
      .bind(own_user_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(count)
  }
}

use std::sync::Arc;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::comment_pub::CommentPub};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait CommentRepo {
  async fn fetch_comments(
    &self,
    post_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<CommentPub>, LogicErr>;
  async fn create_comment(
    &self,
    user_id: &Uuid,
    post_id: &Uuid,
    content_md: &str,
    content_html: &str,
  ) -> Result<Uuid, LogicErr>;
  async fn delete_comment(&self, user_id: &Uuid, post_id: &Uuid, comment_id: &Uuid) -> Result<(), LogicErr>;
  async fn fetch_comments_count(&self, post_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr>;
  async fn create_comment_like(&self, user_id: &Uuid, comment_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr>;
  async fn delete_comment_like(&self, user_id: &Uuid, comment_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr>;
  async fn fetch_comment_count(&self) -> i64;
}

pub type CommentPool = Arc<dyn CommentRepo + Send + Sync>;

pub struct DbCommentRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl CommentRepo for DbCommentRepo {
  async fn fetch_comments(
    &self,
    post_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<CommentPub>, LogicErr> {
    let post = sqlx::query_as(include_str!("./sql/fetch_post_comments.sql"))
      .bind(own_user_id)
      .bind(post_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(post)
  }

  async fn create_comment(
    &self,
    user_id: &Uuid,
    post_id: &Uuid,
    content_md: &str,
    content_html: &str,
  ) -> Result<Uuid, LogicErr> {
    let comment_id = Uuid::new_v4();

    let id = sqlx::query_scalar(
      "INSERT INTO comments (comment_id, user_id, post_id, content_md, content_html) VALUES ($1, $2, $3, $4, $5) RETURNING comment_id",
    )
    .bind(comment_id)
    .bind(user_id)
    .bind(post_id)
    .bind(content_md)
    .bind(content_html)
    .fetch_one(&self.db)
    .await.map_err(map_db_err)?;

    Ok(id)
  }

  async fn delete_comment(&self, user_id: &Uuid, post_id: &Uuid, comment_id: &Uuid) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM comments WHERE post_id = $1 AND user_id = $2 AND comment_id = $3")
      .bind(post_id)
      .bind(user_id)
      .bind(comment_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_comments_count(&self, post_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr> {
    sqlx::query_scalar(include_str!("./sql/fetch_post_comments_count.sql"))
      .bind(own_user_id)
      .bind(post_id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)
  }

  async fn create_comment_like(&self, user_id: &Uuid, comment_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr> {
    let comment_like_id = Uuid::new_v4();

    sqlx::query("INSERT INTO comment_likes (comment_like_id, user_id, comment_id, post_id) VALUES($1, $2, $3, $4)")
      .bind(comment_like_id)
      .bind(user_id)
      .bind(comment_id)
      .bind(post_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_comment_like(&self, user_id: &Uuid, comment_id: &Uuid, post_id: &Uuid) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM comment_likes WHERE user_id = $1 AND comment_id = $2 AND post_id = $3")
      .bind(user_id)
      .bind(comment_id)
      .bind(post_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_comment_count(&self) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM comments")
      .fetch_one(&self.db)
      .await
      .unwrap_or(0)
  }
}

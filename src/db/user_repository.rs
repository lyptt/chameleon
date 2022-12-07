use std::sync::Arc;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::user::User};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepo {
  async fn fetch_by_handle(&self, handle: &str) -> Result<Option<User>, LogicErr>;
  async fn fetch_id_by_handle(&self, handle: &str) -> Option<Uuid>;
  async fn fetch_id_by_fediverse_id(&self, fediverse_id: &str) -> Option<Uuid>;
  async fn fetch_by_fediverse_id(&self, fediverse_id: &str) -> Result<Option<User>, LogicErr>;
  async fn fetch_password_hash(&self, handle: &str) -> Result<Option<String>, LogicErr>;
  async fn fetch_fediverse_id_by_handle(&self, fediverse_id: &str) -> Option<String>;
  async fn fetch_user_count(&self) -> i64;
  async fn fetch_followers(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn fetch_following(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn fetch_followers_count(&self, user_id: &Uuid) -> i64;
  async fn fetch_following_count(&self, user_id: &Uuid) -> i64;
}

pub type UserPool = Arc<dyn UserRepo + Send + Sync>;

pub struct DbUserRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl UserRepo for DbUserRepo {
  async fn fetch_by_handle(&self, handle: &str) -> Result<Option<User>, LogicErr> {
    let user = sqlx::query_as("SELECT * FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(user)
  }

  async fn fetch_id_by_handle(&self, handle: &str) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(&self.db)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  async fn fetch_id_by_fediverse_id(&self, fediverse_id: &str) -> Option<Uuid> {
    match sqlx::query_scalar("SELECT user_id FROM users WHERE fediverse_id = $1")
      .bind(fediverse_id)
      .fetch_optional(&self.db)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  async fn fetch_by_fediverse_id(&self, fediverse_id: &str) -> Result<Option<User>, LogicErr> {
    let user = sqlx::query_as("SELECT * FROM users WHERE fediverse_id = $1")
      .bind(fediverse_id)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(user)
  }

  async fn fetch_password_hash(&self, handle: &str) -> Result<Option<String>, LogicErr> {
    let password_hash = sqlx::query_scalar("SELECT password_hash FROM users WHERE handle = $1")
      .bind(handle)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(password_hash)
  }

  async fn fetch_fediverse_id_by_handle(&self, fediverse_id: &str) -> Option<String> {
    match sqlx::query_scalar("SELECT fediverse_id FROM users WHERE handle = $1")
      .bind(fediverse_id)
      .fetch_optional(&self.db)
      .await
    {
      Ok(user) => user,
      Err(_) => None,
    }
  }

  async fn fetch_user_count(&self) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM users")
      .fetch_one(&self.db)
      .await
      .unwrap_or(0)
  }

  async fn fetch_followers(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr> {
    sqlx::query_as("SELECT u.* FROM users u INNER JOIN followers f ON f.user_id = u.user_id WHERE f.following_user_id = $1 AND f.user_id != following_user_id LIMIT $2 OFFSET $3")
        .bind(user_id)
        .bind(limit)
        .bind(skip)
        .fetch_all(&self.db)
        .await
        .map_err(map_db_err)
  }

  async fn fetch_following(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr> {
    sqlx::query_as("SELECT u.* FROM users u INNER JOIN followers f ON f.following_user_id = u.user_id WHERE f.user_id = $1 AND f.user_id != following_user_id LIMIT $2 OFFSET $3")
        .bind(user_id)
        .bind(limit)
        .bind(skip)
        .fetch_all(&self.db)
        .await
        .map_err(map_db_err)
  }

  async fn fetch_followers_count(&self, user_id: &Uuid) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM followers WHERE following_user_id = $1 AND user_id != following_user_id ")
      .bind(user_id)
      .fetch_one(&self.db)
      .await
      .unwrap_or(0)
  }

  async fn fetch_following_count(&self, user_id: &Uuid) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM followers WHERE user_id = $1 AND user_id != following_user_id ")
      .bind(user_id)
      .fetch_one(&self.db)
      .await
      .unwrap_or(0)
  }
}

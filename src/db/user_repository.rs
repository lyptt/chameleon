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
  async fn fetch_by_id(&self, id: &Uuid) -> Result<User, LogicErr>;
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
  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<User>;
  async fn create(
    &self,
    handle: &str,
    fediverse_id: &str,
    fediverse_uri: &str,
    avatar_url: &Option<String>,
    email: &Option<String>,
    password_hash: &str,
    is_external: bool,
    private_key: &str,
    public_key: &str,
  ) -> Result<Uuid, LogicErr>;
  async fn create_from(&self, user: &User) -> Result<User, LogicErr>;
  async fn delete_user_from_uri(&self, uri: &str) -> Result<(), LogicErr>;
}

pub type UserPool = Arc<dyn UserRepo + Send + Sync>;

pub struct DbUserRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl UserRepo for DbUserRepo {
  async fn fetch_by_id(&self, id: &Uuid) -> Result<User, LogicErr> {
    sqlx::query_as("SELECT * FROM users WHERE user_id = $1")
      .bind(id)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)
  }

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

  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<User> {
    match sqlx::query_as("SELECT * FROM users u WHERE fediverse_uri = $1")
      .bind(fediverse_uri)
      .fetch_optional(&self.db)
      .await
    {
      Ok(u) => u,
      Err(_) => None,
    }
  }

  async fn create(
    &self,
    handle: &str,
    fediverse_id: &str,
    fediverse_uri: &str,
    avatar_url: &Option<String>,
    email: &Option<String>,
    password_hash: &str,
    is_external: bool,
    private_key: &str,
    public_key: &str,
  ) -> Result<Uuid, LogicErr> {
    let user_id = Uuid::new_v4();
    sqlx::query_scalar(r#"INSERT INTO users (user_id, handle, fediverse_id, fediverse_uri, avatar_url, email, password_hash, is_external, private_key, public_key) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING user_id"#)
      .bind(user_id)
      .bind(handle)
      .bind(fediverse_id)
      .bind(fediverse_uri)
      .bind(avatar_url)
      .bind(email)
      .bind(password_hash)
      .bind(is_external)
      .bind(private_key)
      .bind(public_key)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)
  }

  async fn create_from(&self, user: &User) -> Result<User, LogicErr> {
    let user_id: Uuid = sqlx::query_scalar(r#"INSERT INTO users (user_id, handle, fediverse_id, fediverse_uri, avatar_url, email, password_hash, is_external, 
      url_1, url_2, url_3, url_4, url_5, url_1_title, url_2_title, url_3_title, url_4_title, url_5_title, intro_md, intro_html, private_key, public_key, 
      ext_apub_followers_uri, ext_apub_following_uri, ext_apub_inbox_uri, ext_apub_outbox_uri) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26) RETURNING user_id"#)
      .bind(user.user_id)
      .bind(&user.handle)
      .bind(&user.fediverse_id)
      .bind(&user.fediverse_uri)
      .bind(&user.avatar_url)
      .bind(&user.email)
      .bind(&user.password_hash)
      .bind(user.is_external)
      .bind(&user.url_1)
      .bind(&user.url_2)
      .bind(&user.url_3)
      .bind(&user.url_4)
      .bind(&user.url_5)
      .bind(&user.url_1_title)
      .bind(&user.url_2_title)
      .bind(&user.url_3_title)
      .bind(&user.url_4_title)
      .bind(&user.url_5_title)
      .bind(&user.intro_md)
      .bind(&user.intro_html)
      .bind(&user.private_key)
      .bind(&user.public_key)
      .bind(&user.ext_apub_followers_uri)
      .bind(&user.ext_apub_following_uri)
      .bind(&user.ext_apub_inbox_uri)
      .bind(&user.ext_apub_outbox_uri)
      .fetch_one(&self.db)
      .await
      .map_err(map_db_err)?;

    let mut u = user.clone();
    u.user_id = user_id;

    Ok(u)
  }

  async fn delete_user_from_uri(&self, uri: &str) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM users WHERE fediverse_uri = $1")
      .bind(uri)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }
}

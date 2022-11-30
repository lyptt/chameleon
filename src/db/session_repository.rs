use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::{helpers::api::map_db_err, logic::LogicErr};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait SessionRepo {
  async fn query_session_exists_for_refresh_token(&self, refresh_token: &str) -> bool;
  async fn insert_session(
    &self,
    session_id: &Uuid,
    user_id: &Uuid,
    app_id: &Uuid,
    refresh_token: &str,
    access_expires_at: &DateTime<Utc>,
    refresh_expires_at: &DateTime<Utc>,
  ) -> Result<(), LogicErr>;
  async fn delete_session(&self, user_id: &Uuid, app_id: &Uuid, refresh_token: &str) -> Result<(), LogicErr>;
  async fn query_session_exists(&self, session_id: &Uuid) -> bool;
}

pub type SessionPool = Arc<dyn SessionRepo + Send + Sync>;

pub struct DbSessionRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl SessionRepo for DbSessionRepo {
  async fn query_session_exists_for_refresh_token(&self, refresh_token: &str) -> bool {
    let count: i64 =
      match sqlx::query_scalar("SELECT COUNT(*) from sessions WHERE refresh_token = $1 AND refresh_expires_at > NOW()")
        .bind(refresh_token)
        .fetch_one(&self.db)
        .await
      {
        Ok(count) => count,
        Err(err) => {
          println!("{}", err);
          return false;
        }
      };

    count > 0
  }

  async fn insert_session(
    &self,
    session_id: &Uuid,
    user_id: &Uuid,
    app_id: &Uuid,
    refresh_token: &str,
    access_expires_at: &DateTime<Utc>,
    refresh_expires_at: &DateTime<Utc>,
  ) -> Result<(), LogicErr> {
    sqlx::query("INSERT INTO sessions (session_id, user_id, app_id, refresh_token, access_expires_at, refresh_expires_at) VALUES ($1, $2, $3, $4, $5, $6)")
      .bind(session_id)
      .bind(user_id)
      .bind(app_id)
      .bind(refresh_token)
      .bind(access_expires_at)
      .bind(refresh_expires_at)
      .execute(&self.db)
      .await.map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_session(&self, user_id: &Uuid, app_id: &Uuid, refresh_token: &str) -> Result<(), LogicErr> {
    sqlx::query("DELETE FROM sessions WHERE refresh_token = $1 AND app_id = $2 AND user_id = $3")
      .bind(refresh_token)
      .bind(app_id)
      .bind(user_id)
      .execute(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn query_session_exists(&self, session_id: &Uuid) -> bool {
    let count: i64 =
      match sqlx::query_scalar("SELECT COUNT(*) from sessions WHERE session_id = $1 AND access_expires_at > NOW()")
        .bind(session_id)
        .fetch_one(&self.db)
        .await
      {
        Ok(count) => count,
        Err(_) => return false,
      };

    count > 0
  }
}

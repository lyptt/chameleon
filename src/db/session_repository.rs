use async_trait::async_trait;
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::helpers::api::map_db_err;
use crate::logic::LogicErr;

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
  pub db: Pool,
}

#[async_trait]
impl SessionRepo for DbSessionRepo {
  async fn query_session_exists_for_refresh_token(&self, refresh_token: &str) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };

    let row = match db
      .query_one(
        "SELECT COUNT(*) > 0 from sessions WHERE refresh_token = $1 AND refresh_expires_at > NOW()",
        &[&refresh_token],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
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
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      r#"INSERT INTO sessions (session_id, user_id, app_id, refresh_token, access_expires_at, refresh_expires_at)
      VALUES ($1, $2, $3, $4, $5, $6)"#,
      &[
        &session_id,
        &user_id,
        &app_id,
        &refresh_token,
        &access_expires_at,
        &refresh_expires_at,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_session(&self, user_id: &Uuid, app_id: &Uuid, refresh_token: &str) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM sessions WHERE refresh_token = $1 AND app_id = $2 AND user_id = $3",
      &[&refresh_token, &app_id, &user_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn query_session_exists(&self, session_id: &Uuid) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };

    let row = match db
      .query_one(
        "SELECT COUNT(*) > 0 from sessions WHERE session_id = $1 AND access_expires_at > NOW()",
        &[&session_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
  }
}

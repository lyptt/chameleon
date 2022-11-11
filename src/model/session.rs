use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Session {
  pub session_id: Uuid,
  pub user_id: Uuid,
  pub app_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub access_expires_at: DateTime<Utc>,
  pub refresh_expires_at: DateTime<Utc>,
}

impl Session {
  pub async fn insert_session(
    session_id: &Uuid,
    user_id: &Uuid,
    app_id: &Uuid,
    access_expires_at: &DateTime<Utc>,
    refresh_expires_at: &DateTime<Utc>,
    pool: &Pool<Postgres>,
  ) -> Result<(), Error> {
    sqlx::query("INSERT INTO sessions (session_id, user_id, app_id, access_expires_at, refresh_expires_at) VALUES ($1, $2, $3, $4, $5)")
      .bind(session_id)
      .bind(user_id)
      .bind(app_id)
      .bind(access_expires_at)
      .bind(refresh_expires_at)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn query_session_exists(session_id: &Uuid, pool: &Pool<Postgres>) -> Result<bool, Error> {
    todo!()
  }
}

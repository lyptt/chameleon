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
  pub async fn query_session_exists_for_refresh_token(refresh_token: &str, pool: &Pool<Postgres>) -> bool {
    let count: i64 =
      match sqlx::query_scalar("SELECT COUNT(*) from sessions WHERE refresh_token = $1 AND refresh_expires_at > NOW()")
        .bind(refresh_token)
        .fetch_one(pool)
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

  pub async fn insert_session(
    session_id: &Uuid,
    user_id: &Uuid,
    app_id: &Uuid,
    refresh_token: &str,
    access_expires_at: &DateTime<Utc>,
    refresh_expires_at: &DateTime<Utc>,
    pool: &Pool<Postgres>,
  ) -> Result<(), Error> {
    sqlx::query("INSERT INTO sessions (session_id, user_id, app_id, refresh_token, access_expires_at, refresh_expires_at) VALUES ($1, $2, $3, $4, $5, $6)")
      .bind(session_id)
      .bind(user_id)
      .bind(app_id)
      .bind(refresh_token)
      .bind(access_expires_at)
      .bind(refresh_expires_at)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn delete_session(
    user_id: &Uuid,
    app_id: &Uuid,
    refresh_token: &str,
    pool: &Pool<Postgres>,
  ) -> Result<(), Error> {
    sqlx::query("DELETE FROM sessions WHERE refresh_token = $1 AND app_id = $2 AND user_id = $3")
      .bind(refresh_token)
      .bind(app_id)
      .bind(user_id)
      .execute(pool)
      .await?;

    Ok(())
  }

  pub async fn query_session_exists(session_id: &Uuid, pool: &Pool<Postgres>) -> bool {
    let count: i64 =
      match sqlx::query_scalar("SELECT COUNT(*) from sessions WHERE session_id = $1 AND access_expires_at > NOW()")
        .bind(session_id)
        .fetch_one(pool)
        .await
      {
        Ok(count) => count,
        Err(_) => return false,
      };

    count > 0
  }
}

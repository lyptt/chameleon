use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserStats {
  pub following_count: i64,
  pub followers_count: i64,
  pub following_user: bool,
  pub user_is_you: bool,
}

impl UserStats {
  pub async fn fetch_for_user(handle: &str, own_user_id: &Option<Uuid>, pool: &Pool<Postgres>) -> Option<UserStats> {
    let post = sqlx::query_as(include_str!("../db/fetch_user_stats.sql"))
      .bind(handle)
      .bind(own_user_id)
      .fetch_optional(pool)
      .await;

    match post {
      Ok(post) => post,
      Err(_) => None,
    }
  }
}

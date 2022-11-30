use std::sync::Arc;

use crate::model::user_stats::UserStats;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserStatsRepo {
  async fn fetch_for_user(&self, handle: &str, own_user_id: &Option<Uuid>) -> Option<UserStats>;
}

pub type UserStatsPool = Arc<dyn UserStatsRepo + Send + Sync>;

pub struct DbUserStatsRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl UserStatsRepo for DbUserStatsRepo {
  async fn fetch_for_user(&self, handle: &str, own_user_id: &Option<Uuid>) -> Option<UserStats> {
    let post = sqlx::query_as(include_str!("./sql/fetch_user_stats.sql"))
      .bind(handle)
      .bind(own_user_id)
      .fetch_optional(&self.db)
      .await;

    match post {
      Ok(post) => post,
      Err(_) => None,
    }
  }
}

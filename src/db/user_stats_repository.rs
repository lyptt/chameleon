use super::FromRow;
use crate::{helpers::api::map_db_err, model::user_stats::UserStats};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
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
  pub db: Pool,
}

#[async_trait]
impl UserStatsRepo for DbUserStatsRepo {
  async fn fetch_for_user(&self, handle: &str, own_user_id: &Option<Uuid>) -> Option<UserStats> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt(include_str!("./sql/fetch_user_stats.sql"), &[&handle, &own_user_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(UserStats::from_row)
  }
}

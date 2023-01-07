use super::FromRow;
use crate::{helpers::api::map_db_err, logic::LogicErr, model::tombstone::Tombstone};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait TombstoneRepo {
  async fn fetch_for_fediverse_uri(&self, fediverse_uri: &str) -> Option<Tombstone>;
  async fn create_tombstone(&self, fediverse_uri: &str, former_type: &str) -> Result<Uuid, LogicErr>;
}

pub type TombstonePool = Arc<dyn TombstoneRepo + Send + Sync>;

pub struct DbTombstoneRepo {
  pub db: Pool,
}

#[async_trait]
impl TombstoneRepo for DbTombstoneRepo {
  async fn fetch_for_fediverse_uri(&self, fediverse_uri: &str) -> Option<Tombstone> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT * FROM tombstones WHERE fediverse_uri = $1", &[&fediverse_uri])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(Tombstone::from_row)
  }

  async fn create_tombstone(&self, fediverse_uri: &str, former_type: &str) -> Result<Uuid, LogicErr> {
    let tombstone_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "INSERT INTO tombstones (tombstone_id, fediverse_uri, former_type) VALUES ($1, $2, $3) RETURNING tombstone_id",
        &[&tombstone_id, &fediverse_uri, &former_type],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }
}

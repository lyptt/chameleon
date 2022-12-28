use super::FromRow;
use crate::{helpers::api::map_db_err, logic::LogicErr, model::user::User};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait OrbitModeratorRepo {
  async fn count_users(&self, orbit_id: &Uuid) -> Result<i64, LogicErr>;
  async fn fetch_users(&self, orbit_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn user_is_moderator(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<bool, LogicErr>;
  async fn user_is_owner(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<bool, LogicErr>;
  async fn create_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid, is_owner: bool) -> Result<Uuid, LogicErr>;
  async fn update_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid, is_owner: bool) -> Result<(), LogicErr>;
  async fn delete_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr>;
}

pub type OrbitModeratorPool = Arc<dyn OrbitModeratorRepo + Send + Sync>;

pub struct DbOrbitModeratorRepo {
  pub db: Pool,
}

#[async_trait]
impl OrbitModeratorRepo for DbOrbitModeratorRepo {
  async fn count_users(&self, orbit_id: &Uuid) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(u.*) FROM users u INNER JOIN orbit_moderators o ON o.user_id = u.user_id WHERE o.orbit_id = $1",
        &[&orbit_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_users(&self, orbit_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        "SELECT u.* FROM users u INNER JOIN orbit_moderators o ON o.user_id = u.user_id WHERE o.orbit_id = $1 LIMIT $2 OFFSET $3",
        &[&orbit_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(User::from_row).collect())
  }

  async fn user_is_moderator(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<bool, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(*) > 0 FROM orbit_moderators WHERE orbit_id = $1 AND user_id = $2",
        &[&orbit_id, &user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn user_is_owner(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<bool, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(*) > 0 FROM orbit_moderators WHERE orbit_id = $1 AND user_id = $2 AND is_owner = TRUE",
        &[&orbit_id, &user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn create_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid, is_owner: bool) -> Result<Uuid, LogicErr> {
    let orbit_moderator_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "INSERT INTO orbit_moderators (orbit_moderator_id, user_id, orbit_id, is_owner) VALUES ($1, $2, $3, $4) RETURNING orbit_moderator_id",
        &[&orbit_moderator_id, &user_id, &orbit_id, &is_owner],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn update_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid, is_owner: bool) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE orbit_moderators SET is_owner = $1 WHERE orbit_id = $2 AND user_id = $3",
      &[&is_owner, &orbit_id, &user_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_orbit_moderator(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM orbit_moderators WHERE user_id = $1 AND orbit_id = $2",
      &[&user_id, &orbit_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }
}

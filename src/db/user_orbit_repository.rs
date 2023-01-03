use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::user::User};

use super::FromRow;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserOrbitRepo {
  async fn fetch_orbit_user_ids(&self, orbit_id: &Uuid) -> Result<Vec<Uuid>, LogicErr>;
  async fn count_users(&self, orbit_id: &Uuid) -> Result<i64, LogicErr>;
  async fn fetch_users(&self, orbit_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn create_user_orbit(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<Uuid, LogicErr>;
  async fn delete_user_orbit(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr>;
  async fn user_is_member(&self, user_id: &Uuid, orbit_id: &Uuid) -> Result<bool, LogicErr>;
}

pub type UserOrbitPool = Arc<dyn UserOrbitRepo + Send + Sync>;

pub struct DbUserOrbitRepo {
  pub db: Pool,
}

#[async_trait]
impl UserOrbitRepo for DbUserOrbitRepo {
  async fn fetch_orbit_user_ids(&self, orbit_id: &Uuid) -> Result<Vec<Uuid>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        "SELECT u.user_id FROM users u INNER JOIN user_orbits o ON o.user_id = u.user_id WHERE o.orbit_id = $1",
        &[&orbit_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().map(|r| r.get::<&str, Uuid>("user_id")).collect())
  }

  async fn count_users(&self, orbit_id: &Uuid) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(u.*) FROM users u INNER JOIN user_orbits o ON o.user_id = u.user_id WHERE o.orbit_id = $1",
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
        "SELECT u.* FROM users u INNER JOIN user_orbits o ON o.user_id = u.user_id WHERE o.orbit_id = $1 LIMIT $2 OFFSET $3",
        &[&orbit_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(User::from_row).collect())
  }

  async fn create_user_orbit(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<Uuid, LogicErr> {
    let user_orbit_id = Uuid::new_v4();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "INSERT INTO user_orbits (user_orbit_id, user_id, orbit_id) VALUES ($1, $2, $3) RETURNING user_orbit_id",
        &[&user_orbit_id, &user_id, &orbit_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn delete_user_orbit(&self, orbit_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM user_orbits WHERE user_id = $1 AND orbit_id = $2",
      &[&user_id, &orbit_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn user_is_member(&self, user_id: &Uuid, orbit_id: &Uuid) -> Result<bool, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(*) >= 1 FROM user_orbits WHERE user_id = $1 AND orbit_id = $2",
        &[&user_id, &orbit_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }
}

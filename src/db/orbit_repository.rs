use super::FromRow;
use crate::{helpers::api::map_db_err, logic::LogicErr, model::orbit::Orbit};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait OrbitRepo {
  async fn fetch_orbit_id_from_shortcode(&self, shortcode: &str) -> Option<Uuid>;
  async fn count_orbits(&self) -> Result<i64, LogicErr>;
  async fn fetch_orbits(&self, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr>;
  async fn fetch_orbit(&self, orbit_id: &Uuid) -> Result<Option<Orbit>, LogicErr>;
  async fn count_user_orbits(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  async fn fetch_user_orbits(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr>;
  async fn create_orbit(
    &self,
    name: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
  ) -> Result<Uuid, LogicErr>;
  async fn update_orbit(
    &self,
    orbit_id: &Uuid,
    name: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
  ) -> Result<(), LogicErr>;
  async fn orbit_is_external(&self, orbit_id: &Uuid) -> Result<bool, LogicErr>;
  async fn update_orbit_from(&self, orbit: &Orbit) -> Result<(), LogicErr>;
  async fn delete_orbit(&self, orbit_id: &Uuid) -> Result<(), LogicErr>;
}

pub type OrbitPool = Arc<dyn OrbitRepo + Send + Sync>;

pub struct DbOrbitRepo {
  pub db: Pool,
}

#[async_trait]
impl OrbitRepo for DbOrbitRepo {
  async fn fetch_orbit_id_from_shortcode(&self, shortcode: &str) -> Option<Uuid> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt(
        "SELECT orbit_id FROM orbits WHERE shortcode = $1 AND is_external = FALSE",
        &[&shortcode],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(|r| r.get(0))
  }

  async fn count_orbits(&self) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one("SELECT COUNT(*) FROM orbits WHERE is_external = FALSE", &[])
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_orbits(&self, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        "SELECT * FROM orbits WHERE is_external = FALSE LIMIT $1 OFFSET $2",
        &[&limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(Orbit::from_row).collect())
  }

  async fn count_user_orbits(&self, user_id: &Uuid) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        "SELECT COUNT(o.*) FROM orbits o INNER JOIN user_orbits u ON u.orbit_id = o.orbit_id WHERE u.user_id = $1",
        &[&user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_user_orbits(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        "SELECT o.* FROM orbits o INNER JOIN user_orbits u ON u.orbit_id = o.orbit_id WHERE u.user_id = $1 LIMIT $2 OFFSET $3",
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(Orbit::from_row).collect())
  }

  async fn fetch_orbit(&self, orbit_id: &Uuid) -> Result<Option<Orbit>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT * FROM orbits WHERE orbit_id = $1", &[&orbit_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(Orbit::from_row))
  }

  async fn create_orbit(
    &self,
    name: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
  ) -> Result<Uuid, LogicErr> {
    let orbit_id = Uuid::new_v4();
    let uri = format!("/orbits/{}", orbit_id);

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        r#"INSERT INTO orbits (orbit_id, name, description_md, description_html, avatar_uri, banner_uri, uri, is_external)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING orbit_id"#,
        &[&orbit_id, &name, &description_md, &description_html, &avatar_uri, &banner_uri, &uri, &is_external],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn update_orbit(
    &self,
    orbit_id: &Uuid,
    name: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
  ) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE orbits SET name = $2, description_md = $3, description_html = $4, avatar_uri = $5, banner_uri = $6, is_external = $7 WHERE orbit_id = $1",
      &[&orbit_id, &name, &description_md, &description_html, &avatar_uri, &banner_uri, &is_external],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn orbit_is_external(&self, orbit_id: &Uuid) -> Result<bool, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one("SELECT is_external FROM orbits WHERE orbit_id = $1", &[&orbit_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn update_orbit_from(&self, orbit: &Orbit) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE orbits SET name = $2, description_md = $3, description_html = $4, avatar_uri = $5, banner_uri = $6, uri = $7, is_external = $8 WHERE orbit_id = $1",
      &[&orbit.orbit_id, &orbit.name, &orbit.description_md, &orbit.description_html, &orbit.avatar_uri, &orbit.banner_uri, &orbit.uri, &orbit.is_external],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_orbit(&self, orbit_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute("DELETE FROM orbits WHERE orbit_id = $1", &[&orbit_id])
      .await
      .map_err(map_db_err)?;

    Ok(())
  }
}

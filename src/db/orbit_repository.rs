use super::FromRow;
use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{orbit::Orbit, orbit_pub::OrbitPub},
};

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
  async fn fetch_orbit_shortcode_from_id(&self, id: &Uuid) -> Option<String>;
  async fn count_orbits(&self) -> Result<i64, LogicErr>;
  async fn fetch_orbits(&self, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr>;
  async fn fetch_orbit(&self, orbit_id: &Uuid) -> Result<Option<Orbit>, LogicErr>;
  async fn fetch_orbit_for_user(&self, orbit_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<OrbitPub>, LogicErr>;
  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<Orbit>;
  async fn count_user_orbits(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  async fn fetch_user_orbits(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<Orbit>, LogicErr>;
  async fn fetch_popular_orbits(&self) -> Result<Vec<Orbit>, LogicErr>;
  async fn create_orbit(
    &self,
    name: &str,
    shortcode: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
    priv_key: &str,
    pub_key: &str,
    uri: &str,
  ) -> Result<Uuid, LogicErr>;
  async fn create_from(&self, orbit: &Orbit) -> Result<Orbit, LogicErr>;
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
  async fn delete_external_orbit(&self, orbit_id: &Uuid) -> Result<(), LogicErr>;
  async fn fetch_outdated_external_orbits(&self) -> Result<Vec<Uuid>, LogicErr>;
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

  async fn fetch_orbit_shortcode_from_id(&self, id: &Uuid) -> Option<String> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt(
        "SELECT shortcode FROM orbits WHERE orbit_id = $1 AND is_external = FALSE",
        &[&id],
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
        "SELECT o.* FROM orbits o INNER JOIN user_orbits u ON u.orbit_id = o.orbit_id WHERE u.user_id = $1 ORDER BY o.shortcode ASC LIMIT $2 OFFSET $3",
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(Orbit::from_row).collect())
  }

  async fn fetch_popular_orbits(&self) -> Result<Vec<Orbit>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        r#"SELECT o.* FROM orbits o
      INNER JOIN posts p
      ON p.orbit_id = o.orbit_id
      GROUP BY o.orbit_id
      ORDER BY COUNT(p.*) DESC
      LIMIT 10"#,
        &[],
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

  async fn fetch_orbit_for_user(&self, orbit_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<OrbitPub>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt(
        r#"SELECT o.*, COUNT(uo.*) >= 1 AS joined, COUNT(om.*) >= 1 AS moderating
      FROM orbits o
      LEFT OUTER JOIN user_orbits uo
      ON uo.orbit_id = o.orbit_id AND uo.user_id = $2
      LEFT OUTER JOIN orbit_moderators om
      ON om.orbit_id = o.orbit_id AND om.user_id = $2
      WHERE o.orbit_id = $1
      GROUP BY o.orbit_id"#,
        &[&orbit_id, &user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(OrbitPub::from_row))
  }

  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<Orbit> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_opt("SELECT * FROM orbits WHERE fediverse_uri = $1", &[&fediverse_uri])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(Orbit::from_row)
  }

  async fn create_orbit(
    &self,
    name: &str,
    shortcode: &str,
    description_md: &str,
    description_html: &str,
    avatar_uri: &Option<String>,
    banner_uri: &Option<String>,
    is_external: bool,
    priv_key: &str,
    pub_key: &str,
    uri: &str,
  ) -> Result<Uuid, LogicErr> {
    let orbit_id = Uuid::new_v4();
    let fediverse_uri = format!("/orbit/{}", orbit_id);

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        r#"INSERT INTO orbits (orbit_id, name, description_md, description_html, avatar_uri, banner_uri, uri, fediverse_uri, is_external, shortcode, private_key, public_key)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING orbit_id"#,
        &[&orbit_id, &name, &description_md, &description_html, &avatar_uri, &banner_uri, &uri, &fediverse_uri, &is_external, &shortcode, &priv_key, &pub_key],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn create_from(&self, orbit: &Orbit) -> Result<Orbit, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db
      .execute(
        r#"INSERT INTO orbits (orbit_id, shortcode, name, description_md, description_html, avatar_uri, banner_uri, uri, fediverse_uri, is_external, ext_apub_inbox_uri, ext_apub_outbox_uri, ext_apub_followers_uri, private_key, public_key)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
        &[&orbit.orbit_id, &orbit.shortcode, &orbit.name, &orbit.description_md, &orbit.description_html, &orbit.avatar_uri, &orbit.banner_uri, &orbit.uri, &orbit.fediverse_uri, &orbit.is_external, &orbit.ext_apub_inbox_uri, &orbit.ext_apub_outbox_uri, &orbit.ext_apub_followers_uri, &orbit.private_key, &orbit.public_key],
      )
      .await
      .map_err(map_db_err)?;

    Ok(orbit.to_owned())
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
      "UPDATE orbits SET name = $2, description_md = $3, description_html = $4, avatar_uri = $5, banner_uri = $6, is_external = $7, updated_at = NOW() WHERE orbit_id = $1",
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
      "UPDATE orbits SET shortcode = $2, name = $3, description_md = $4, description_html = $5, avatar_uri = $6, banner_uri = $7, uri = $8, fediverse_uri = $9, private_key = $10, public_key = $11, is_external = $12, ext_apub_inbox_uri = $13, ext_apub_outbox_uri = $14, ext_apub_followers_uri = $15, updated_at = NOW() WHERE orbit_id = $1",
      &[
        &orbit.orbit_id,
        &orbit.shortcode,
        &orbit.name,
        &orbit.description_md,
        &orbit.description_html,
        &orbit.avatar_uri,
        &orbit.banner_uri,
        &orbit.uri,
        &orbit.fediverse_uri,
        &orbit.private_key,
        &orbit.public_key,
        &orbit.is_external,
        &orbit.ext_apub_inbox_uri,
        &orbit.ext_apub_outbox_uri,
        &orbit.ext_apub_followers_uri
      ],
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

  async fn delete_external_orbit(&self, orbit_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM orbits WHERE orbit_id = $1 AND is_external = TRUE",
      &[&orbit_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_outdated_external_orbits(&self) -> Result<Vec<Uuid>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        r#"SELECT orbit_id FROM orbits WHERE updated_at < NOW() - INTERVAL '30 minutes'"#,
        &[],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().map(|r| r.get::<&str, Uuid>("orbit_id")).collect())
  }
}

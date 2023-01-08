use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

use super::orbit::Orbit;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct OrbitPub {
  pub orbit_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub shortcode: String,
  pub name: String,
  pub description_md: String,
  pub description_html: String,
  pub avatar_uri: Option<String>,
  pub banner_uri: Option<String>,
  pub fediverse_id: String,
  pub uri: String,
  pub is_external: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub joined: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub moderating: Option<bool>,
}

impl FromRow for OrbitPub {
  fn from_row(row: Row) -> Option<Self> {
    Some(OrbitPub {
      orbit_id: row.get("orbit_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      shortcode: row.get("shortcode"),
      name: row.get("name"),
      description_md: row.get("description_md"),
      description_html: row.get("description_html"),
      avatar_uri: row.get("avatar_uri"),
      banner_uri: row.get("banner_uri"),
      fediverse_id: row.get("fediverse_id"),
      uri: row.get("uri"),
      is_external: row.get("is_external"),
      joined: row.get("joined"),
      moderating: row.get("moderating"),
    })
  }
}

impl From<Orbit> for OrbitPub {
  fn from(orbit: Orbit) -> Self {
    OrbitPub {
      orbit_id: orbit.orbit_id,
      created_at: orbit.created_at,
      updated_at: orbit.updated_at,
      shortcode: orbit.shortcode,
      name: orbit.name,
      description_md: orbit.description_md,
      description_html: orbit.description_html,
      avatar_uri: orbit.avatar_uri,
      banner_uri: orbit.banner_uri,
      fediverse_id: orbit.fediverse_id,
      uri: orbit.uri,
      is_external: orbit.is_external,
      joined: None,
      moderating: None,
    }
  }
}

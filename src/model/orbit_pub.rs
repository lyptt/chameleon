use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

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
  pub uri: String,
  pub is_external: bool,
  pub joined: bool,
  pub moderating: bool,
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
      uri: row.get("uri"),
      is_external: row.get("is_external"),
      joined: row.get("joined"),
      moderating: row.get("moderating"),
    })
  }
}

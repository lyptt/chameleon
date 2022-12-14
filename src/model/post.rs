use crate::db::FromRow;

use super::access_type::AccessType;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Post {
  pub post_id: Uuid,
  pub user_id: Uuid,
  pub orbit_id: Option<Uuid>,
  pub uri: String,
  pub is_external: bool,
  pub title: Option<String>,
  pub content_md: String,
  pub content_html: String,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deletion_scheduled_at: Option<DateTime<Utc>>,
}

impl FromRow for Post {
  fn from_row(row: Row) -> Option<Self> {
    Some(Post {
      post_id: row.get("post_id"),
      user_id: row.get("user_id"),
      orbit_id: row.get("orbit_id"),
      uri: row.get("uri"),
      is_external: row.get("is_external"),
      title: row.get("title"),
      content_md: row.get("content_md"),
      content_html: row.get("content_html"),
      visibility: AccessType::from_str(row.get("visibility")).unwrap_or_default(),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      deletion_scheduled_at: row.get("deletion_scheduled_at"),
    })
  }
}

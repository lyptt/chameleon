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
  pub uri: String,
  pub is_external: bool,
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
      uri: row.get("uri"),
      is_external: row.get("is_external"),
      content_md: row.get("content_md"),
      content_html: row.get("content_html"),
      visibility: AccessType::from_str(row.get("visibility")).unwrap_or_default(),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      deletion_scheduled_at: row.get("deletion_scheduled_at"),
    })
  }
}

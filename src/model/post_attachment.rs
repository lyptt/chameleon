use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::{FromRow, FromRowJoin};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
/// Represents a user's follow on another user
pub struct PostAttachment {
  pub attachment_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uri: Option<String>,
  pub width: i32,
  pub height: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub storage_ref: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blurhash: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl FromRow for PostAttachment {
  fn from_row(row: Row) -> Option<Self> {
    Some(PostAttachment {
      attachment_id: row.get("attachment_id"),
      user_id: row.get("user_id"),
      post_id: row.get("post_id"),
      uri: row.get("uri"),
      width: row.get("width"),
      height: row.get("height"),
      content_type: row.get("content_type"),
      storage_ref: row.get("storage_ref"),
      blurhash: row.get("blurhash"),
      created_at: row.get("created_at"),
    })
  }
}

impl FromRowJoin for PostAttachment {
  fn from_row_join(row: &Row) -> Option<Self> {
    Some(PostAttachment {
      attachment_id: row.get("attachment_id"),
      user_id: row.get("attachment_user_id"),
      post_id: row.get("attachment_post_id"),
      uri: row.get("attachment_uri"),
      width: row.get("attachment_width"),
      height: row.get("attachment_height"),
      content_type: row.get("attachment_content_type"),
      storage_ref: row.get("attachment_storage_ref"),
      blurhash: row.get("attachment_blurhash"),
      created_at: row.get("attachment_created_at"),
    })
  }
}

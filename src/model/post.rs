use super::access_type::AccessType;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Post {
  pub post_id: Uuid,
  pub user_id: Uuid,
  pub uri: String,
  pub is_external: bool,
  pub content_md: String,
  pub content_html: String,
  pub content_image_uri_small: Option<String>,
  pub content_image_uri_medium: Option<String>,
  pub content_image_uri_large: Option<String>,
  pub content_width_small: Option<i32>,
  pub content_width_medium: Option<i32>,
  pub content_width_large: Option<i32>,
  pub content_height_small: Option<i32>,
  pub content_height_medium: Option<i32>,
  pub content_height_large: Option<i32>,
  pub content_type_small: Option<String>,
  pub content_type_medium: Option<String>,
  pub content_type_large: Option<String>,
  pub content_image_storage_ref: Option<String>,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub deletion_scheduled_at: Option<DateTime<Utc>>,
}

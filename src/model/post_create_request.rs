use super::access_type::AccessType;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostCreateRequest {
  pub is_external: bool,
  pub content_md: String,
  pub visibility: AccessType,
}

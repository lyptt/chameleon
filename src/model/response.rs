use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse<T> {
  pub data: Vec<T>,
  pub page: i64,
  pub total_items: i64,
  pub total_pages: i64,
}

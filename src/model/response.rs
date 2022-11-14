use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct ListResponse<T> {
  pub data: Vec<T>,
  pub page: i64,
  pub total_items: i64,
  pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobResponse {
  pub job_id: Uuid,
}

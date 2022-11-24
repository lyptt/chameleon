use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ListResponse<T> {
  pub data: Vec<T>,
  pub page: i64,
  pub total_items: i64,
  pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct JobResponse {
  pub job_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ObjectResponse<T> {
  pub data: T,
}

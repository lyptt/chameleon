use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, PartialEq, Eq, Debug, Clone)]
pub struct App {
  pub app_id: Uuid,
  pub name: String,
  pub description: String,
  pub owner_name: String,
  pub owner_uri: String,
  pub redirect_uri: String,
  pub client_id: String,
  pub client_secret: String,
}

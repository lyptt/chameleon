use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct App {
  pub app_id: Uuid,
  pub user_id: Uuid,
  pub name: String,
  pub description: String,
  pub owner_name: String,
  pub owner_uri: String,
  pub owner_instance_uri: String,
  pub redirect_uri: String,
  pub blessed: bool,
  pub client_id: Uuid,
  pub client_secret: Uuid,
}

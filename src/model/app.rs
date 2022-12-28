use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::db::FromRow;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
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

impl FromRow for App {
  fn from_row(row: Row) -> Option<Self> {
    Some(App {
      app_id: row.get("app_id"),
      name: row.get("name"),
      description: row.get("description"),
      owner_name: row.get("owner_name"),
      owner_uri: row.get("owner_uri"),
      redirect_uri: row.get("redirect_uri"),
      client_id: row.get("client_id"),
      client_secret: row.get("client_secret"),
    })
  }
}

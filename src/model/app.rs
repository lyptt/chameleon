use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
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

impl App {
  pub async fn fetch_by_client_id(client_id: &str, pool: &Pool<Postgres>) -> Result<Option<App>, Error> {
    let client_uuid = match Uuid::parse_str(client_id) {
      Ok(uuid) => uuid,
      Err(_) => return Err(Error::ColumnNotFound("client_uuid".to_string())),
    };

    let app = sqlx::query_as("SELECT * FROM apps WHERE client_id = $1")
      .bind(client_uuid)
      .fetch_optional(pool)
      .await?;

    Ok(app)
  }
}

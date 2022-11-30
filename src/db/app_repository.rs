use std::sync::Arc;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::app::App};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr>;
}

pub type AppPool = Arc<dyn AppRepo + Send + Sync>;

pub struct DbAppRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl AppRepo for DbAppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr> {
    let client_uuid = match Uuid::parse_str(client_id) {
      Ok(uuid) => uuid,
      Err(_) => return Err(LogicErr::MissingRecord),
    };

    let app = sqlx::query_as("SELECT * FROM apps WHERE client_id = $1")
      .bind(client_uuid)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(app)
  }
}

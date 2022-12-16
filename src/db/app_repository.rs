use std::sync::Arc;

use crate::{helpers::api::map_db_err, logic::LogicErr, model::app::App};

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[cfg(test)]
use mockall::automock;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr>;
  async fn create(&self, app: &App) -> Result<(), LogicErr>;
}

pub type AppPool = Arc<dyn AppRepo + Send + Sync>;

pub struct DbAppRepo {
  pub db: Pool<Postgres>,
}

#[async_trait]
impl AppRepo for DbAppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr> {
    let app = sqlx::query_as("SELECT * FROM apps WHERE client_id = $1")
      .bind(client_id)
      .fetch_optional(&self.db)
      .await
      .map_err(map_db_err)?;

    Ok(app)
  }

  async fn create(&self, app: &App) -> Result<(), LogicErr> {
    sqlx::query("INSERT INTO apps (app_id, name, description, owner_name, owner_uri, redirect_uri, client_id, client_secret) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
      .bind(app.app_id)
      .bind(&app.name)
      .bind(&app.description)
      .bind(&app.owner_name)
      .bind(&app.owner_uri)
      .bind(&app.redirect_uri)
      .bind(&app.client_id)
      .bind(&app.client_secret)
      .execute(&self.db)
      .await
      .map(|_| Ok(()))
      .map_err(map_db_err)?
  }
}

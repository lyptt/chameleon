use crate::{helpers::api::map_db_err, logic::LogicErr, model::app::App};

use async_trait::async_trait;
use std::sync::Arc;

use deadpool_postgres::Pool;
#[cfg(test)]
use mockall::automock;

use super::FromRow;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait AppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr>;
  async fn create(&self, app: &App) -> Result<(), LogicErr>;
}

pub type AppPool = Arc<dyn AppRepo + Send + Sync>;

pub struct DbAppRepo {
  pub db: Pool,
}

#[async_trait]
impl AppRepo for DbAppRepo {
  async fn fetch_by_client_id(&self, client_id: &str) -> Result<Option<App>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT * FROM apps WHERE client_id = $1", &[&client_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(App::from_row))
  }

  async fn create(&self, app: &App) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      r#"INSERT INTO apps (app_id, name, description, owner_name, owner_uri, redirect_uri, client_id, client_secret)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
      &[
        &app.app_id,
        &app.name,
        &app.description,
        &app.owner_name,
        &app.owner_uri,
        &app.redirect_uri,
        &app.client_id,
        &app.client_secret,
      ],
    )
    .await
    .map_err(map_db_err)?;
    Ok(())
  }
}

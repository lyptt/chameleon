use rand::distributions::{Alphanumeric, DistString};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::LogicErr;
use crate::{db::app_repository::AppPool, model::app::App};

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct NewApp {
  name: String,
  description: String,
  owner_name: String,
  owner_uri: String,
  redirect_uri: String,
}

pub async fn create_app(apps: &AppPool, new_app: &NewApp) -> Result<App, LogicErr> {
  let mut client_id_hasher = Sha256::new();
  let client_id_data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
  client_id_hasher.update(client_id_data.as_bytes());

  let mut client_secret_hasher = Sha256::new();
  let client_secret_data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
  client_secret_hasher.update(client_secret_data.as_bytes());

  let client_id = hex::encode(client_id_hasher.finalize());
  let client_secret = hex::encode(client_secret_hasher.finalize());

  let app = App {
    app_id: Uuid::new_v4(),
    name: new_app.name.clone(),
    description: new_app.description.clone(),
    owner_name: new_app.owner_name.clone(),
    owner_uri: new_app.owner_uri.clone(),
    redirect_uri: new_app.redirect_uri.clone(),
    blessed: false,
    client_id,
    client_secret,
  };

  apps.create(&app).await.map(|_| Ok(app))?
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use mockall::predicate::*;

  use crate::{
    db::app_repository::{AppPool, MockAppRepo},
    logic::{
      app::{create_app, NewApp},
      LogicErr,
    },
  };

  #[async_std::test]
  async fn test_create_app_rejects_db_err_passthrough() {
    let new_app = NewApp {
      name: "a".to_string(),
      description: "a".to_string(),
      owner_name: "a".to_string(),
      owner_uri: "a".to_string(),
      redirect_uri: "a".to_string(),
    };

    let mut app_repo = MockAppRepo::new();

    app_repo
      .expect_create()
      .times(1)
      .with(always())
      .return_const(Err(LogicErr::DbError("Failed".to_string())));

    let apps: AppPool = Arc::new(app_repo);

    assert_eq!(
      create_app(&apps, &new_app).await,
      Err(LogicErr::DbError("Failed".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_app_succeeds() {
    let new_app = NewApp {
      name: "a".to_string(),
      description: "a".to_string(),
      owner_name: "a".to_string(),
      owner_uri: "a".to_string(),
      redirect_uri: "a".to_string(),
    };

    let mut app_repo = MockAppRepo::new();

    app_repo.expect_create().times(1).with(always()).return_const(Ok(()));

    let apps: AppPool = Arc::new(app_repo);

    assert!(create_app(&apps, &new_app).await.is_ok());
  }
}

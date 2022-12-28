use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use deadpool_postgres::Pool;
use serde::Serialize;

use crate::helpers::api::map_ext_err;

#[derive(Debug, Serialize)]
struct ServerComponentStatus {
  ok: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  error: Option<String>,
  updated_at: i64,
}

#[derive(Debug, Serialize)]
struct ServerStatus {
  db: ServerComponentStatus,
}

pub async fn api_get_server_status(db: web::Data<Pool>) -> impl Responder {
  // TODO: Implement me
  let (db_status, db_error) = match db.get().await {
    Ok(db) => match db.execute("SELECT 1", &[]).await {
      Ok(_) => (true, None),
      Err(err) => (false, Some(map_ext_err(err))),
    },
    Err(err) => (false, Some(map_ext_err(err))),
  };

  HttpResponse::Ok().json(ServerStatus {
    db: ServerComponentStatus {
      ok: db_status,
      error: db_error.map(|e| e.to_string()),
      updated_at: Utc::now().timestamp(),
    },
  })
}

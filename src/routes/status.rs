use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerComponentStatus {
  ok: bool,
  error: Option<String>,
  updated_at: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerStatus {
  db: ServerComponentStatus,
}
#[utoipa::path(
  get,
  path = "/.well-known/status",
  responses(
      (status = 200, description = "Success", body = ServerStatus),
  ),
)]
pub async fn api_get_server_status(db: web::Data<PgPool>) -> impl Responder {
  let db_status = match sqlx::query_scalar("SELECT 1").fetch_one(&**db).await {
    Ok::<i32, _>(_) => (true, None),
    Err(err) => (false, Some(err.to_string())),
  };

  HttpResponse::Ok().json(ServerStatus {
    db: ServerComponentStatus {
      ok: db_status.0,
      error: db_status.1,
      updated_at: Utc::now().timestamp(),
    },
  })
}

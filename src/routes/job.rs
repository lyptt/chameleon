use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
  db::{job_repository::JobPool, session_repository::SessionPool},
  helpers::{auth::query_auth, core::map_api_err},
  logic::job::fetch_job,
  net::jwt::JwtContext,
};

pub async fn api_job_query_status(
  sessions: web::Data<SessionPool>,
  jobs: web::Data<JobPool>,
  job_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = query_auth(&jwt, &sessions).await;

  match fetch_job(&jobs, &job_id, &props.map(|v| v.uid)).await {
    Ok(job) => HttpResponse::Ok().json(job),
    Err(err) => map_api_err(err),
  }
}

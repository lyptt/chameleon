use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  helpers::{
    auth::query_auth,
    core::{build_api_err, build_api_not_found},
  },
  model::job::Job,
  net::jwt::JwtContext,
};

pub async fn api_job_query_status(
  db: web::Data<PgPool>,
  job_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = query_auth(&jwt, &db).await;

  match Job::fetch_by_id(&job_id, &db).await {
    Ok(job) => match job {
      Some(job) => match job.created_by_id {
        Some(created_by_id) => match props {
          Some(props) => match created_by_id == props.uid {
            true => HttpResponse::Ok().json(job),
            false => build_api_not_found(job_id.to_string()),
          },
          None => build_api_not_found(job_id.to_string()),
        },
        None => HttpResponse::Ok().json(job),
      },
      None => build_api_not_found(job_id.to_string()),
    },
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

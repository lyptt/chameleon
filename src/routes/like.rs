use crate::{
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::like::{create_like, delete_like},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[utoipa::path(
  post,
  path = "/api/feed/{post_id}/likes",
  responses(
      (status = 200, description = "Success", body = WebfingerRecord),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Query, description = "The post you're liking")
  )
)]
pub async fn api_create_like(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_like(&db, &post_id, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

#[utoipa::path(
  delete,
  path = "/api/feed/{post_id}/likes",
  responses(
      (status = 200, description = "Success", body = WebfingerRecord),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Query, description = "The post you're unliking")
  )
)]
pub async fn api_delete_like(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_like(&db, &post_id, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

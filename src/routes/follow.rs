use crate::{
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::follow::{create_follow, delete_follow},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn api_create_follow(
  db: web::Data<PgPool>,
  user_handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_follow(&db, &user_handle, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_follow(
  db: web::Data<PgPool>,
  user_handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_follow(&db, &user_handle, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

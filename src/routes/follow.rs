use crate::{
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::follow::{create_follow, delete_follow},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[utoipa::path(
  post,
  path = "/api/users/{user_handle}/follows",
  responses(
      (status = 201, description = "Follow created"),
      (status = 401, description = "Unauthorized"),
      (status = 500, description = "Internal server error")
  ),
  params(
      ("user_handle" = Uuid, Path, description = "ID of the user you wish to follow"),
  )
)]
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

#[utoipa::path(
  delete,
  path = "/api/users/{user_handle}/follows",
  responses(
      (status = 200, description = "Follow created"),
      (status = 401, description = "Unauthorized"),
      (status = 500, description = "Internal server error")
  ),
  params(
      ("user_handle" = Uuid, Path, description = "ID of the user you wish to unfollow"),
  )
)]
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

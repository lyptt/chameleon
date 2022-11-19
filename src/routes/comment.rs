use crate::{
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::comment::{create_comment, delete_comment},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewPost {
  content_md: String,
}

pub async fn api_create_comment(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  contents: web::Json<NewPost>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_comment(&db, &post_id, &props.uid, &contents.content_md).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_comment(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  comment_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_comment(&db, &post_id, &comment_id, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

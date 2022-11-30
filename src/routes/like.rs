use crate::{
  db::{
    follow_repository::FollowPool, like_repository::LikePool, post_repository::PostPool,
    session_repository::SessionPool,
  },
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::like::{create_like, delete_like},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

pub async fn api_create_like(
  sessions: web::Data<SessionPool>,
  follows: web::Data<FollowPool>,
  posts: web::Data<PostPool>,
  likes: web::Data<LikePool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_like(&posts, &follows, &likes, &post_id, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_like(
  sessions: web::Data<SessionPool>,
  likes: web::Data<LikePool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_like(&likes, &post_id, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

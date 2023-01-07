use crate::{
  db::{
    follow_repository::FollowPool, job_repository::JobPool, session_repository::SessionPool, user_repository::UserPool,
  },
  helpers::auth::require_auth,
  helpers::core::build_api_err,
  logic::follow::{create_follow, delete_follow},
  net::jwt::JwtContext,
  work_queue::queue::Queue,
};
use actix_web::{web, HttpResponse, Responder};

pub async fn api_create_follow(
  sessions: web::Data<SessionPool>,
  follows: web::Data<FollowPool>,
  users: web::Data<UserPool>,
  jobs: web::Data<JobPool>,
  queue: web::Data<Queue>,
  user_handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_follow(&users, &follows, &jobs, &queue, &user_handle, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_follow(
  sessions: web::Data<SessionPool>,
  follows: web::Data<FollowPool>,
  users: web::Data<UserPool>,
  jobs: web::Data<JobPool>,
  queue: web::Data<Queue>,
  user_handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_follow(&users, &follows, &jobs, &queue, &user_handle, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

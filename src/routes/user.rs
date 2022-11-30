use actix_web::{web, HttpResponse, Responder};

use crate::{
  db::{session_repository::SessionPool, user_repository::UserPool, user_stats_repository::UserStatsPool},
  helpers::{
    activitypub::{handle_async_activitypub_alt_get, handle_async_activitypub_get},
    api::result_into,
    auth::{query_auth, require_auth},
  },
  logic::user::{get_user_by_fediverse_id, get_user_by_id},
  model::{response::ObjectResponse, user_account_pub::UserAccountPub},
  net::jwt::JwtContext,
};

pub async fn api_activitypub_get_user_by_id_astream(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
) -> impl Responder {
  handle_async_activitypub_alt_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &users).await))
}

pub async fn api_activitypub_get_user_by_id(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &users).await))
}

pub async fn api_get_profile(
  sessions: web::Data<SessionPool>,
  users: web::Data<UserPool>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match get_user_by_fediverse_id(&props.sub, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_profile(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_id(&handle, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_stats(
  sessions: web::Data<SessionPool>,
  user_stats: web::Data<UserStatsPool>,
  jwt: web::ReqData<JwtContext>,
  handle: web::Path<String>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match user_stats.fetch_for_user(&handle, &own_user_id).await {
    Some(user) => HttpResponse::Ok().json(ObjectResponse { data: user }),
    None => HttpResponse::NotFound().finish(),
  }
}

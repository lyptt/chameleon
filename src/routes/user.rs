use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
  helpers::{
    activitypub::{handle_async_activitypub_alt_get, handle_async_activitypub_get},
    api::result_into,
    auth::{query_auth, require_auth},
  },
  logic::user::{get_user_by_fediverse_id, get_user_by_id},
  model::{response::ObjectResponse, user_account_pub::UserAccountPub, user_stats::UserStats},
  net::jwt::JwtContext,
};

pub async fn api_activitypub_get_user_by_id_astream(
  db: web::Data<PgPool>,
  handle: web::Path<String>,
) -> impl Responder {
  handle_async_activitypub_alt_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &db).await))
}

pub async fn api_activitypub_get_user_by_id(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &db).await))
}

pub async fn api_get_profile(db: web::Data<PgPool>, jwt: web::ReqData<JwtContext>) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match get_user_by_fediverse_id(&props.sub, &db).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_profile(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_id(&handle, &db).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_stats(
  db: web::Data<PgPool>,
  jwt: web::ReqData<JwtContext>,
  handle: web::Path<String>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &db).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match UserStats::fetch_for_user(&handle, &own_user_id, &db).await {
    Some(user) => HttpResponse::Ok().json(ObjectResponse { data: user }),
    None => HttpResponse::NotFound().finish(),
  }
}

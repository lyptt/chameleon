use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
  helpers::{
    activitypub::{handle_async_activitypub_alt_get, handle_async_activitypub_get},
    api::result_into,
    auth::require_auth,
  },
  logic::user::{get_user_by_fediverse_id, get_user_by_id},
  model::user_account_pub::UserAccountPub,
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

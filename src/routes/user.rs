use actix_web::{web, Responder};
use sqlx::PgPool;

use crate::{
  helpers::handlers::{handle_async_activitypub_alt_get, handle_async_activitypub_get},
  logic::user::get_user_by_id,
};

pub async fn api_get_user_by_id_astream(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_alt_get(&handle, &get_user_by_id(&handle, &db).await)
}

pub async fn api_get_user_by_id(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_get(&handle, &get_user_by_id(&handle, &db).await)
}

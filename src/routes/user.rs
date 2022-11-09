use actix_web::{web, Responder};
use sqlx::PgPool;

use crate::{
  helpers::handlers::{handle_async_activitypub_alt_get, handle_async_activitypub_get, result_into},
  logic::user::get_user_by_id,
  model::user_account_pub::UserAccountPub,
};

pub async fn api_get_user_by_id_astream(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_alt_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &db).await))
}

pub async fn api_get_user_by_id(db: web::Data<PgPool>, handle: web::Path<String>) -> impl Responder {
  handle_async_activitypub_get::<UserAccountPub>(&handle, &result_into(get_user_by_id(&handle, &db).await))
}

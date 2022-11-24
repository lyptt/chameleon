use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
  helpers::core::{build_api_err, build_api_not_found},
  logic::user::get_user_by_fediverse_id,
};

#[derive(Debug, Deserialize)]
pub struct WebfingerQuery {
  resource: String,
}

#[utoipa::path(
  get,
  path = "/.well-known/webfinger",
  responses(
      (status = 200, description = "Success", body = WebfingerRecord),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("resource" = String, Query, description = "The resource you're querying information on")
  )
)]
pub async fn api_webfinger_query_resource(db: web::Data<PgPool>, query: web::Query<WebfingerQuery>) -> impl Responder {
  match get_user_by_fediverse_id(&query.resource, &db).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(user.to_webfinger()),
      None => build_api_not_found(query.resource.to_string()),
    },
    Err(err) => build_api_err(1, err.to_string(), None),
  }
}

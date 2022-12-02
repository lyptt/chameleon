use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
  db::user_repository::UserPool,
  helpers::core::{build_api_err, build_api_not_found},
  logic::user::get_user_by_fediverse_id,
};

#[derive(Debug, Deserialize)]
pub struct WebfingerQuery {
  resource: String,
}

pub async fn api_webfinger_query_resource(
  users: web::Data<UserPool>,
  query: web::Query<WebfingerQuery>,
) -> impl Responder {
  match get_user_by_fediverse_id(&query.resource, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(user.to_webfinger()),
      None => build_api_not_found(query.resource.to_string()),
    },
    Err(err) => build_api_err(1, err.to_string(), None),
  }
}

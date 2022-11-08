use actix_web::HttpResponse;

use super::{
  core::{build_api_err, build_api_not_found},
  types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE},
};
use crate::logic::LogicErr;

fn handle_async_get<T: serde::Serialize>(
  source: &str,
  result: &Result<Option<T>, LogicErr>,
  content_type: &str,
) -> HttpResponse {
  match result {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok()
        .insert_header(("Content-Type", content_type))
        .json(user),
      None => build_api_not_found(source.to_string()),
    },
    Err(err) => build_api_err(1, err.to_string(), Some(source.to_string())),
  }
}

pub fn handle_async_activitypub_get<T: serde::Serialize>(
  source: &str,
  result: &Result<Option<T>, LogicErr>,
) -> HttpResponse {
  handle_async_get(source, result, &ACTIVITY_JSON_CONTENT_TYPE)
}

pub fn handle_async_activitypub_alt_get<T: serde::Serialize>(
  source: &str,
  result: &Result<Option<T>, LogicErr>,
) -> HttpResponse {
  handle_async_get(source, result, &ACTIVITY_LD_JSON_CONTENT_TYPE)
}

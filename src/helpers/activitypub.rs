use actix_web::HttpResponse;

use super::{
  api::handle_async_get,
  core::build_api_err,
  types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE},
};
use crate::{activitypub::ordered_collection::OrderedCollection, logic::LogicErr};

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

pub fn handle_activitypub_collection_metadata_get(
  subject_url: &str,
  page_size: i64,
  result: Result<i64, LogicErr>,
) -> HttpResponse {
  match result {
    Ok(total_items) => HttpResponse::Ok()
      .insert_header(("Content-Type", *ACTIVITY_JSON_CONTENT_TYPE))
      .json(OrderedCollection::build(&subject_url, total_items, page_size)),
    Err(err) => build_api_err(1, err.to_string(), Some(subject_url.to_string())),
  }
}

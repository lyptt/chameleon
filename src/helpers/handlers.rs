use actix_web::HttpResponse;
use num_traits::PrimInt;
use std::error::Error;

use super::{
  core::{build_api_err, build_api_not_found},
  types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE},
};
use crate::{activitypub::ordered_collection::OrderedCollection, logic::LogicErr};

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

pub fn result_into<A, B: serde::Serialize + std::convert::From<A>>(
  a: Result<Option<A>, LogicErr>,
) -> Result<Option<B>, LogicErr> {
  a.map(|a| a.map(|a| a.into()))
}

pub fn map_ext_err<A: Error>(err: A) -> LogicErr {
  LogicErr::InternalError(err.to_string())
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

pub fn div_up<T: PrimInt>(a: T, b: T) -> T {
  let whole = a / b;
  let part = a % b;

  match part > T::from(0).unwrap() && whole >= T::from(1).unwrap() {
    true => whole + T::from(1).unwrap(),
    false => whole,
  }
}

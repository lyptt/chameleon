use actix_web::HttpResponse;
use std::error::Error;

use super::core::{build_api_err, build_api_not_found};
use crate::{logic::LogicErr, settings::SETTINGS};

pub fn handle_async_get<T: serde::Serialize>(
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

pub fn map_db_err<A: Error>(err: A) -> LogicErr {
  LogicErr::DbError(err.to_string())
}

pub fn relative_to_absolute_uri(relative: &str) -> String {
  match relative.starts_with("http") {
    true => relative.to_string(),
    false => format!("{}{}", SETTINGS.server.api_fqdn, relative),
  }
}

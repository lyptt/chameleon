use actix_web::{web, HttpResponse};
use num_traits::PrimInt;
use serde::Serialize;
use std::error::Error;

use super::{
  core::{build_api_err, build_api_not_found},
  types::{ACTIVITY_JSON_CONTENT_TYPE, ACTIVITY_LD_JSON_CONTENT_TYPE},
};
use crate::{
  activitypub::ordered_collection::OrderedCollection,
  logic::LogicErr,
  model::app::App,
  net::{
    jwt::{JwtContext, JwtContextProps},
    templates::HANDLEBARS,
  },
};

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

#[derive(Debug, Serialize)]
struct OAuthAuthorizeErrData<'a> {
  pub error: String,
  pub username: Option<&'a str>,
  pub blessed: bool,
  pub app_name: Option<&'a str>,
}

pub fn handle_oauth_app_err(err: &'static str) -> HttpResponse {
  match HANDLEBARS.render(
    "oauth_authorize_app_err",
    &OAuthAuthorizeErrData {
      error: err.to_string(),
      username: None,
      blessed: false,
      app_name: None,
    },
  ) {
    Ok(body) => return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body),
    Err(_) => return HttpResponse::InternalServerError().finish(),
  };
}

pub fn handle_oauth_app_body(app: &App, err: &'static str) -> HttpResponse {
  match HANDLEBARS.render(
    "oauth_authorize",
    &OAuthAuthorizeErrData {
      error: err.to_string(),
      username: None,
      blessed: app.blessed,
      app_name: Some(&app.name),
    },
  ) {
    Ok(body) => return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body),
    Err(_) => return HttpResponse::InternalServerError().finish(),
  };
}

pub fn oauth_app_unwrap_result<T>(obj: Result<Option<T>, sqlx::Error>, error: &'static str) -> Result<T, HttpResponse> {
  match obj {
    Ok(obj) => match obj {
      Some(obj) => Ok(obj),
      None => Err(handle_oauth_app_err(error)),
    },
    Err(err) => {
      println!("{}", err);
      Err(handle_oauth_app_err(error))
    }
  }
}

pub fn assert_auth(jwt: &web::ReqData<JwtContext>) -> Result<(), HttpResponse> {
  match (**jwt).clone() {
    JwtContext::Valid(_) => Ok(()),
    JwtContext::Invalid(_) => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub fn require_auth(jwt: &web::ReqData<JwtContext>) -> Result<JwtContextProps, HttpResponse> {
  match (**jwt).clone() {
    JwtContext::Valid(props) => Ok(props),
    JwtContext::Invalid(_) => Err(HttpResponse::Unauthorized().finish()),
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

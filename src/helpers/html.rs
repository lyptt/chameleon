use actix_web::HttpResponse;
use serde::Serialize;

use crate::{model::app::App, net::templates::HANDLEBARS};

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

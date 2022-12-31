use actix_web::HttpResponse;
use serde::Serialize;

use crate::{logic::LogicErr, model::app::App, net::templates::HANDLEBARS, settings::SETTINGS};

#[derive(Debug, Serialize)]
struct OAuthAuthorizeErrData<'a> {
  pub error: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<&'a str>,
  pub blessed: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub app_name: Option<&'a str>,
  pub orbit_name: &'a str,
}

pub fn build_orbit_name() -> String {
  SETTINGS.server.fqdn.replace("https://", "").replace("http://", "")
}

pub fn handle_oauth_app_err(err: &'static str) -> HttpResponse {
  match HANDLEBARS.render(
    "oauth_authorize_app_err",
    &OAuthAuthorizeErrData {
      error: err.to_string(),
      username: None,
      blessed: false,
      app_name: None,
      orbit_name: &build_orbit_name(),
    },
  ) {
    Ok(body) => return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub fn handle_oauth_app_body(app: &App, blessed: bool, err: &str) -> HttpResponse {
  match HANDLEBARS.render(
    "oauth_authorize",
    &OAuthAuthorizeErrData {
      error: err.to_string(),
      username: None,
      blessed,
      app_name: Some(&app.name),
      orbit_name: &build_orbit_name(),
    },
  ) {
    Ok(body) => return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}

pub fn oauth_app_unwrap_result<T>(obj: Result<Option<T>, LogicErr>, error: &'static str) -> Result<T, HttpResponse> {
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

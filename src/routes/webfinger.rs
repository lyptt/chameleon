use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
  db::{orbit_repository::OrbitPool, user_repository::UserPool},
  helpers::core::{build_api_err, build_api_not_found},
  logic::user::get_user_by_webfinger,
  settings::SETTINGS,
};

#[derive(Debug, Deserialize)]
pub struct WebfingerQuery {
  resource: String,
}

pub async fn api_webfinger_query_resource(
  users: web::Data<UserPool>,
  orbits: web::Data<OrbitPool>,
  query: web::Query<WebfingerQuery>,
) -> impl Responder {
  if query.resource.starts_with("acct") {
    return match get_user_by_webfinger(&query.resource, &users).await {
      Ok(user) => match user {
        Some(user) => HttpResponse::Ok().json(user.to_webfinger()),
        None => build_api_not_found(query.resource.to_string()),
      },
      Err(err) => build_api_err(1, err.to_string(), None),
    };
  }

  let components: Vec<&str> = query.resource.splitn(2, ':').collect();

  if components.len() != 2 {
    return build_api_not_found(query.resource.to_owned());
  }

  let scheme = components[0];
  let location = components[1];
  let location_components: Vec<&str> = location.splitn(2, '@').collect();

  if location_components.len() != 2 {
    return build_api_not_found(query.resource.to_owned());
  }

  let name = location_components[0];
  let location = location_components[1];

  if !SETTINGS.server.api_root_fqdn.contains(location) {
    return build_api_not_found(query.resource.to_owned());
  }

  if scheme == "group" {
    let orbit_id = match orbits.fetch_orbit_id_from_shortcode(name).await {
      Some(id) => id,
      None => return build_api_not_found(query.resource.to_owned()),
    };

    return match orbits.fetch_orbit(&orbit_id).await {
      Ok(orbit) => match orbit {
        Some(orbit) => HttpResponse::Ok().json(orbit.to_webfinger()),
        None => build_api_not_found(query.resource.to_string()),
      },
      Err(err) => build_api_err(1, err.to_string(), None),
    };
  }

  build_api_not_found(query.resource.to_owned())
}

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use substring::Substring;

use crate::{
  db::{orbit_repository::OrbitPool, user_repository::UserPool},
  federation::activitypub::actor::{federate_orbit_group_from_webfinger, federate_user_actor_from_webfinger},
  helpers::core::map_api_err,
  model::{orbit_pub::OrbitPub, response::ListResponse, user_account_pub::UserAccountPub},
};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
  pub term: String,
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

pub async fn api_search(
  query: web::Query<SearchQuery>,
  users: web::Data<UserPool>,
  orbits: web::Data<OrbitPool>,
) -> impl Responder {
  if query.term.starts_with('@') {
    let components: Vec<&str> = query.term.substring(1, query.term.len()).splitn(2, '@').collect();
    if components.is_empty() {
      return HttpResponse::Ok().json(ListResponse::<UserAccountPub> {
        data: vec![],
        total_items: 0,
        total_pages: 1,
        page: 0,
      });
    }

    if components.len() == 1 {
      match users.fetch_by_handle(components[0]).await {
        Ok(user) => match user {
          Some(user) => HttpResponse::Ok().json(ListResponse {
            data: vec![UserAccountPub::from(user)],
            total_items: 1,
            total_pages: 1,
            page: 0,
          }),
          None => HttpResponse::Ok().json(ListResponse::<UserAccountPub> {
            data: vec![],
            total_items: 0,
            total_pages: 1,
            page: 0,
          }),
        },
        Err(err) => map_api_err(err),
      }
    } else {
      match users.fetch_by_fediverse_id(&query.term).await {
        Ok(user) => match user {
          Some(user) => HttpResponse::Ok().json(ListResponse {
            data: vec![UserAccountPub::from(user)],
            total_items: 1,
            total_pages: 1,
            page: 0,
          }),
          None => {
            match federate_user_actor_from_webfinger(
              components[1],
              &format!("acct:{}@{}", components[0], components[1]),
              &users,
            )
            .await
            {
              Ok(user) => match user {
                Some(user) => HttpResponse::Ok().json(ListResponse {
                  data: vec![UserAccountPub::from(user)],
                  total_items: 1,
                  total_pages: 1,
                  page: 0,
                }),
                None => HttpResponse::Ok().json(ListResponse::<UserAccountPub> {
                  data: vec![],
                  total_items: 0,
                  total_pages: 1,
                  page: 0,
                }),
              },
              Err(err) => map_api_err(err),
            }
          }
        },
        Err(err) => map_api_err(err),
      }
    }
  } else if query.term.starts_with("o/") {
    let components: Vec<&str> = query.term.splitn(2, '@').collect();
    if components.is_empty() {
      return HttpResponse::Ok().json(ListResponse::<OrbitPub> {
        data: vec![],
        total_items: 0,
        total_pages: 1,
        page: 0,
      });
    }

    if components.len() == 1 {
      match orbits.fetch_orbit_from_shortcode(components[0]).await {
        Ok(orbit) => match orbit {
          Some(orbit) => HttpResponse::Ok().json(ListResponse {
            data: vec![OrbitPub::from(orbit)],
            total_items: 1,
            total_pages: 1,
            page: 0,
          }),
          None => HttpResponse::Ok().json(ListResponse::<UserAccountPub> {
            data: vec![],
            total_items: 0,
            total_pages: 1,
            page: 0,
          }),
        },
        Err(err) => map_api_err(err),
      }
    } else {
      match orbits.fetch_by_fediverse_id(&query.term).await {
        Some(orbit) => HttpResponse::Ok().json(ListResponse {
          data: vec![OrbitPub::from(orbit)],
          total_items: 1,
          total_pages: 1,
          page: 0,
        }),
        None => {
          match federate_orbit_group_from_webfinger(
            components[1],
            &format!("group:{}@{}", components[0], components[1]),
            &orbits,
          )
          .await
          {
            Ok(orbit) => match orbit {
              Some(orbit) => HttpResponse::Ok().json(ListResponse {
                data: vec![OrbitPub::from(orbit)],
                total_items: 1,
                total_pages: 1,
                page: 0,
              }),
              None => HttpResponse::Ok().json(ListResponse::<OrbitPub> {
                data: vec![],
                total_items: 0,
                total_pages: 1,
                page: 0,
              }),
            },
            Err(err) => map_api_err(err),
          }
        }
      }
    }
  } else {
    HttpResponse::NotFound().finish()
  }
}

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible, document::ActivityPubDocument,
    helpers::create_activitypub_ordered_collection_page,
  },
  db::{session_repository::SessionPool, user_repository::UserPool, user_stats_repository::UserStatsPool},
  helpers::{
    auth::{query_auth, require_auth},
    core::{build_api_err, build_api_not_found},
    math::div_up,
    types::ACTIVITY_JSON_CONTENT_TYPE,
  },
  logic::user::{get_user_by_fediverse_id, get_user_by_handle},
  model::{
    response::{ListResponse, ObjectResponse},
    user_account_pub::UserAccountPub,
  },
  net::jwt::JwtContext,
  settings::SETTINGS,
};

pub async fn api_get_profile(
  sessions: web::Data<SessionPool>,
  users: web::Data<UserPool>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match get_user_by_fediverse_id(&props.sub, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_profile(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_handle(&handle, &users).await {
    Ok(user) => match user {
      Some(user) => HttpResponse::Ok().json(UserAccountPub::from(user)),
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_activitypub_get_user_profile(users: web::Data<UserPool>, handle: web::Path<String>) -> impl Responder {
  match get_user_by_handle(&handle, &users).await {
    Ok(user) => match user {
      Some(user) => match user.to_object("") {
        Some(obj) => {
          let doc = ActivityPubDocument::new(obj);
          HttpResponse::Ok()
            .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
            .json(doc)
        }
        None => HttpResponse::NotFound().finish(),
      },
      None => HttpResponse::NotFound().finish(),
    },
    Err(_) => HttpResponse::NotFound().finish(),
  }
}

pub async fn api_get_user_stats(
  sessions: web::Data<SessionPool>,
  user_stats: web::Data<UserStatsPool>,
  jwt: web::ReqData<JwtContext>,
  handle: web::Path<String>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  match user_stats.fetch_for_user(&handle, &own_user_id).await {
    Some(user) => HttpResponse::Ok().json(ObjectResponse { data: user }),
    None => HttpResponse::NotFound().finish(),
  }
}

#[derive(Debug, Deserialize)]
pub struct FollowersQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

pub async fn api_get_user_followers(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_followers_count(&user_id).await;

  match users.fetch_followers(&user_id, page_size, page * page_size).await {
    Ok(users) => HttpResponse::Ok().json(ListResponse {
      data: users.into_iter().map(UserAccountPub::from).collect(),
      page,
      total_items: users_count,
      total_pages: div_up(users_count, page_size) + 1,
    }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_get_user_following(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_following_count(&user_id).await;

  match users.fetch_following(&user_id, page_size, page * page_size).await {
    Ok(users) => HttpResponse::Ok().json(ListResponse {
      data: users.into_iter().map(UserAccountPub::from).collect(),
      page,
      total_items: users_count,
      total_pages: div_up(users_count, page_size) + 1,
    }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

pub async fn api_activitypub_get_user_followers(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_followers_count(&user_id).await;

  let users = match users.fetch_followers(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/users/{}/followers", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    users_count.try_into().unwrap_or_default(),
    users,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

pub async fn api_activitypub_get_user_following(
  users: web::Data<UserPool>,
  handle: web::Path<String>,
  query: web::Query<FollowersQuery>,
) -> impl Responder {
  let user_id = match users.fetch_id_by_handle(&handle).await {
    Some(id) => id,
    None => return build_api_not_found(handle.to_string()),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let users_count = users.fetch_following_count(&user_id).await;

  let users = match users.fetch_following(&user_id, page_size, page * page_size).await {
    Ok(users) => users,
    Err(err) => return build_api_err(500, err.to_string(), None),
  };

  let doc = create_activitypub_ordered_collection_page(
    &format!("{}/users/{}/following", SETTINGS.server.api_fqdn, handle),
    page.try_into().unwrap_or_default(),
    page_size.try_into().unwrap_or_default(),
    users_count.try_into().unwrap_or_default(),
    users,
  );

  HttpResponse::Ok()
    .insert_header(("Content-Type", ACTIVITY_JSON_CONTENT_TYPE))
    .json(doc)
}

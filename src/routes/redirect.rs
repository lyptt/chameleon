use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{db::user_repository::UserPool, helpers::core::build_api_not_found, settings::SETTINGS};

pub async fn api_redirect_to_post(post_id: web::Path<Uuid>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/feed/{}", SETTINGS.server.fqdn, post_id)))
    .finish()
}

pub async fn api_redirect_to_post_comments(post_id: web::Path<Uuid>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/feed/{}", SETTINGS.server.fqdn, post_id)))
    .finish()
}

pub async fn api_redirect_to_post_comment(ids: web::Path<(Uuid, Uuid)>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/feed/{}", SETTINGS.server.fqdn, ids.0)))
    .finish()
}

pub async fn api_redirect_to_federated_user_posts(
  user_id: web::Path<Uuid>,
  users: web::Data<UserPool>,
) -> impl Responder {
  let handle = match users.fetch_by_id(&user_id).await {
    Ok(user) => user.handle,
    Err(_) => return build_api_not_found(user_id.to_string()),
  };

  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}", SETTINGS.server.fqdn, handle)))
    .finish()
}

pub async fn api_redirect_to_federated_user_liked_posts(
  user_id: web::Path<Uuid>,
  users: web::Data<UserPool>,
) -> impl Responder {
  let handle = match users.fetch_by_id(&user_id).await {
    Ok(user) => user.handle,
    Err(_) => return build_api_not_found(user_id.to_string()),
  };

  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}/liked", SETTINGS.server.fqdn, handle)))
    .finish()
}

pub async fn api_redirect_to_user_followers(user_id: web::Path<Uuid>, users: web::Data<UserPool>) -> impl Responder {
  let handle = match users.fetch_by_id(&user_id).await {
    Ok(user) => user.handle,
    Err(_) => return build_api_not_found(user_id.to_string()),
  };

  HttpResponse::Found()
    .append_header((
      "location",
      format!("{}/users/{}/followers", SETTINGS.server.fqdn, handle),
    ))
    .finish()
}

pub async fn api_redirect_to_user_following(user_id: web::Path<Uuid>, users: web::Data<UserPool>) -> impl Responder {
  let handle = match users.fetch_by_id(&user_id).await {
    Ok(user) => user.handle,
    Err(_) => return build_api_not_found(user_id.to_string()),
  };

  HttpResponse::Found()
    .append_header((
      "location",
      format!("{}/users/{}/following", SETTINGS.server.fqdn, handle),
    ))
    .finish()
}

pub async fn api_redirect_to_user(user_id: web::Path<Uuid>, users: web::Data<UserPool>) -> impl Responder {
  let handle = match users.fetch_by_id(&user_id).await {
    Ok(user) => user.handle,
    Err(_) => return build_api_not_found(user_id.to_string()),
  };

  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}", SETTINGS.server.fqdn, handle)))
    .finish()
}

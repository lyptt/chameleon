use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::settings::SETTINGS;

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

pub async fn api_redirect_to_federated_user_posts(handle: web::Path<String>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}", SETTINGS.server.fqdn, handle)))
    .finish()
}

pub async fn api_redirect_to_federated_user_liked_posts(handle: web::Path<String>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}/liked", SETTINGS.server.fqdn, handle)))
    .finish()
}

pub async fn api_redirect_to_user_followers(handle: web::Path<String>) -> impl Responder {
  HttpResponse::Found()
    .append_header((
      "location",
      format!("{}/users/{}/followers", SETTINGS.server.fqdn, handle),
    ))
    .finish()
}

pub async fn api_redirect_to_user_following(handle: web::Path<String>) -> impl Responder {
  HttpResponse::Found()
    .append_header((
      "location",
      format!("{}/users/{}/following", SETTINGS.server.fqdn, handle),
    ))
    .finish()
}

pub async fn api_redirect_to_user(handle: web::Path<String>) -> impl Responder {
  HttpResponse::Found()
    .append_header(("location", format!("{}/users/{}", SETTINGS.server.fqdn, handle)))
    .finish()
}

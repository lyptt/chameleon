use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  cdn::cdn_store::Cdn,
  helpers::{core::build_api_err, handlers::handle_activitypub_collection_metadata_get},
  logic::post::get_user_posts_count,
  model::access_type::AccessType,
  settings::SETTINGS,
};

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[derive(MultipartForm)]
pub struct PostUpload {
  #[multipart(rename = "images[]")]
  images: Vec<Tempfile>,
}

pub async fn api_get_user_public_feed(
  db: web::Data<PgPool>,
  handle: web::Path<String>,
  query: web::Query<PostsQuery>,
) -> impl Responder {
  match query.page {
    Some(_page) => HttpResponse::BadRequest().finish(),
    None => handle_activitypub_collection_metadata_get(
      &format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, handle),
      query.page_size.unwrap_or(20),
      get_user_posts_count(&handle, vec![AccessType::PublicLocal, AccessType::PublicFederated], &db).await,
    ),
  }
}

pub async fn api_upload_post_image(form: MultipartForm<PostUpload>, cdn: web::Data<Cdn>) -> impl Responder {
  if form.images.is_empty() {
    return HttpResponse::BadRequest().finish();
  }

  let file_name = format!("originals/{}", Uuid::new_v4());

  match cdn.upload_file(&form.images[0], &file_name).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(1, err.to_string(), None),
  }
}

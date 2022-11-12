use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  activitypub::ordered_collection::OrderedCollectionPage,
  cdn::cdn_store::Cdn,
  helpers::{
    activitypub::handle_activitypub_collection_metadata_get, auth::require_auth, core::build_api_err, math::div_up,
  },
  logic::post::{get_global_posts, get_global_posts_count, get_user_posts, get_user_posts_count},
  model::response::ListResponse,
  net::jwt::JwtContext,
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

pub async fn api_activitypub_get_user_public_feed(
  db: web::Data<PgPool>,
  handle: web::Path<String>,
  query: web::Query<PostsQuery>,
) -> impl Responder {
  match query.page {
    Some(page) => {
      let page_size = query.page_size.unwrap_or(20);
      let posts_count = match get_user_posts_count(&handle, &db).await {
        Ok(count) => count,
        Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
      };

      let posts = match get_user_posts(&handle, page_size, page * page_size, &db).await {
        Ok(posts) => posts,
        Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
      };

      let collection = OrderedCollectionPage::build(
        &format!(
          "{}/users/{}/feed?page={}&page_size={}",
          SETTINGS.server.api_fqdn, &handle, page, page_size
        ),
        &format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, &handle),
        &format!("{}/users/{}/status", SETTINGS.server.api_fqdn, &handle),
        &format!("{}/users/{}", SETTINGS.server.api_fqdn, &handle),
        posts,
        posts_count,
        page_size,
        page,
      );

      HttpResponse::Ok().json(collection)
    }
    None => handle_activitypub_collection_metadata_get(
      &format!("{}/users/{}/feed", SETTINGS.server.api_fqdn, handle),
      query.page_size.unwrap_or(20),
      get_user_posts_count(&handle, &db).await,
    ),
  }
}

pub async fn api_get_user_own_feed(
  db: web::Data<PgPool>,
  query: web::Query<PostsQuery>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  let fediverse_id = props.sub;
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_user_posts_count(&fediverse_id, &db).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_user_posts(&fediverse_id, page_size, page * page_size, &db).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
}

pub async fn api_get_global_feed(db: web::Data<PgPool>, query: web::Query<PostsQuery>) -> impl Responder {
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_global_posts_count(&db).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_global_posts(page_size, page * page_size, &db).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: posts,
    page,
    total_items: posts_count,
    total_pages: div_up(posts_count, page_size) + 1,
  })
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

use actix_easy_multipart::{tempfile::Tempfile, MultipartForm};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
  activitypub::ordered_collection::OrderedCollectionPage,
  cdn::cdn_store::Cdn,
  helpers::{
    activitypub::handle_activitypub_collection_metadata_get,
    auth::{query_auth, require_auth},
    core::{build_api_err, build_api_not_found},
    math::div_up,
  },
  logic::post::{
    create_post, get_global_posts, get_global_posts_count, get_post, get_user_posts, get_user_posts_count,
    upload_post_file, NewPostRequest, NewPostResponse,
  },
  model::{
    access_type::AccessType,
    follow::Follow,
    post_event::PostEvent,
    response::{JobResponse, ListResponse, ObjectResponse},
    user::User,
  },
  net::jwt::JwtContext,
  settings::SETTINGS,
  work_queue::queue::Queue,
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
  let user_id = match User::fetch_id_by_handle(&handle, &db).await {
    Some(id) => id,
    None => return build_api_not_found("No such id".to_string()),
  };

  match query.page {
    Some(page) => {
      let page_size = query.page_size.unwrap_or(20);
      let posts_count = match get_user_posts_count(&user_id, &db).await {
        Ok(count) => count,
        Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
      };

      let posts = match get_user_posts(&user_id, page_size, page * page_size, &db).await {
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
      get_user_posts_count(&user_id, &db).await,
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

  let user_id = props.uid;
  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match get_user_posts_count(&user_id, &db).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match get_user_posts(&user_id, page_size, page * page_size, &db).await {
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

pub async fn api_get_post(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let current_user_id = match query_auth(&jwt, &db).await {
    Some(props) => User::fetch_id_by_fediverse_id(&props.sub, &db).await,
    None => None,
  };

  let post = match get_post(&post_id, &current_user_id, &db).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return build_api_not_found(post_id.to_string()),
    },
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if post.visibility == AccessType::PublicFederated
    || post.visibility == AccessType::PublicLocal
    || post.visibility == AccessType::Unlisted
  {
    return HttpResponse::Ok().json(ObjectResponse { data: post });
  }

  match current_user_id {
    Some(current_user_id) => {
      if post.user_id == current_user_id {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      if post.visibility == AccessType::FollowersOnly
        && Follow::user_follows_poster(&post.post_id, &current_user_id, &db).await
      {
        return HttpResponse::Ok().json(ObjectResponse { data: post });
      }

      HttpResponse::NotFound().finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
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

pub async fn api_get_user_posts(
  db: web::Data<PgPool>,
  query: web::Query<PostsQuery>,
  handle: web::Path<String>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let user_id = match query_auth(&jwt, &db).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  let target_id = match User::fetch_id_by_handle(&handle, &db).await {
    Some(id) => id,
    None => return HttpResponse::NotFound().finish(),
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let posts_count = match PostEvent::count_user_public_feed(&target_id, &user_id, &db).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  let posts = match PostEvent::fetch_user_public_feed(&target_id, &user_id, page_size, page * page_size, &db).await {
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

pub async fn api_create_post(
  db: web::Data<PgPool>,
  req: web::Json<NewPostRequest>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_post(&db, &req, &props.uid).await {
    Ok(post_id) => HttpResponse::Ok().json(NewPostResponse { id: post_id }),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_upload_post_image(
  form: MultipartForm<PostUpload>,
  post_id: web::Path<Uuid>,
  cdn: web::Data<Cdn>,
  queue: web::Data<Queue>,
  db: web::Data<PgPool>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  if form.images.is_empty() {
    return HttpResponse::BadRequest().finish();
  }

  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match upload_post_file(&db, &post_id, &props.uid, &cdn, &queue, &form.images[0]).await {
    Ok(job_id) => HttpResponse::Ok().json(JobResponse { job_id }),
    Err(err) => build_api_err(500, err.to_string(), None),
  }
}

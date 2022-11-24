use crate::{
  helpers::auth::{query_auth, require_auth},
  helpers::core::build_api_err,
  helpers::math::div_up,
  logic::comment::{create_comment, create_comment_like, delete_comment, delete_comment_like},
  model::{comment::Comment, comment_pub::CommentPub, response::ListResponse},
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewPost {
  content_md: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentsQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[utoipa::path(
  post,
  path = "/api/feed/{post_id}/comments",
  responses(
      (status = 201, description = "Success"),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Query, description = "The post you're commenting on")
  ),
  request_body(content = NewPost, content_type = "application/json")
)]
pub async fn api_create_comment(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  contents: web::Json<NewPost>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_comment(&db, &post_id, &props.uid, &contents.content_md).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

#[utoipa::path(
  delete,
  path = "/api/feed/{post_id}/comments/{comment_id}",
  responses(
      (status = 200, description = "Success"),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Query, description = "The post your comment is on"),
    ("comment_id" = String, Query, description = "The id of your comment")
  ),
  request_body(content = NewPost, content_type = "application/json")
)]
pub async fn api_delete_comment(
  db: web::Data<PgPool>,
  post_id: web::Path<Uuid>,
  comment_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_comment(&db, &post_id, &comment_id, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

#[utoipa::path(
  get,
  path = "/api/feed/{post_id}/comments",
  responses(
      (status = 200, description = "Success", body = ListResponse<CommentPub>),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 404, description = "Not found", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Path, description = "The post you're querying for comments on")
  ),
)]
pub async fn api_get_comments(
  db: web::Data<PgPool>,
  query: web::Query<CommentsQuery>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &db).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let comments_count = match Comment::fetch_comments_count(&post_id, &own_user_id, &db).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if comments_count == 0 {
    return HttpResponse::NotFound().finish();
  }

  let comments = match CommentPub::fetch_comments(&post_id, &own_user_id, page_size, page * page_size, &db).await {
    Ok(posts) => posts,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  HttpResponse::Ok().json(ListResponse {
    data: comments,
    page,
    total_items: comments_count,
    total_pages: div_up(comments_count, page_size) + 1,
  })
}

#[utoipa::path(
  post,
  path = "/api/feed/{post_id}/comments/{comment_id}/likes",
  responses(
      (status = 200, description = "Success", body = ListResponse<CommentPub>),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 404, description = "Not found", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Path, description = "The post your comment is on"),
    ("comment_id" = String, Path, description = "The comment you're liking")
  ),
)]
pub async fn api_create_comment_like(
  db: web::Data<PgPool>,
  ids: web::Path<(Uuid, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_comment_like(&db, &ids.0, &ids.1, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

#[utoipa::path(
  delete,
  path = "/api/feed/{post_id}/comments/{comment_id}/likes",
  responses(
      (status = 200, description = "Success", body = ListResponse<CommentPub>),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 404, description = "Not found", body = ApiError),
      (status = 500, description = "Internal server error", body = ApiError)
  ),
  params(
    ("post_id" = String, Path, description = "The post your comment is on"),
    ("comment_id" = String, Path, description = "The comment you're unliking")
  ),
)]
pub async fn api_delete_comment_like(
  db: web::Data<PgPool>,
  ids: web::Path<(Uuid, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &db).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_comment_like(&db, &ids.0, &ids.1, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

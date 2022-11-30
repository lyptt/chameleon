use crate::{
  db::{
    comment_repository::CommentPool, follow_repository::FollowPool, post_repository::PostPool,
    session_repository::SessionPool,
  },
  helpers::auth::{query_auth, require_auth},
  helpers::core::build_api_err,
  helpers::math::div_up,
  logic::comment::{create_comment, create_comment_like, delete_comment, delete_comment_like},
  model::response::ListResponse,
  net::jwt::JwtContext,
};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
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

pub async fn api_create_comment(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  follows: web::Data<FollowPool>,
  posts: web::Data<PostPool>,
  post_id: web::Path<Uuid>,
  contents: web::Json<NewPost>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_comment(&posts, &follows, &comments, &post_id, &props.uid, &contents.content_md).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_comment(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  post_id: web::Path<Uuid>,
  comment_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_comment(&comments, &post_id, &comment_id, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_get_comments(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  query: web::Query<CommentsQuery>,
  post_id: web::Path<Uuid>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let own_user_id = match query_auth(&jwt, &sessions).await {
    Some(props) => Some(props.uid),
    None => None,
  };

  let page = query.page.unwrap_or(0);
  let page_size = query.page_size.unwrap_or(20);
  let comments_count = match comments.fetch_comments_count(&post_id, &own_user_id).await {
    Ok(count) => count,
    Err(err) => return build_api_err(500, err.to_string(), Some(err.to_string())),
  };

  if comments_count == 0 {
    return HttpResponse::NotFound().finish();
  }

  let comments = match comments
    .fetch_comments(&post_id, &own_user_id, page_size, page * page_size)
    .await
  {
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

pub async fn api_create_comment_like(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  follows: web::Data<FollowPool>,
  posts: web::Data<PostPool>,
  ids: web::Path<(Uuid, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match create_comment_like(&posts, &follows, &comments, &ids.0, &ids.1, &props.uid).await {
    Ok(_) => HttpResponse::Created().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

pub async fn api_delete_comment_like(
  sessions: web::Data<SessionPool>,
  comments: web::Data<CommentPool>,
  ids: web::Path<(Uuid, Uuid)>,
  jwt: web::ReqData<JwtContext>,
) -> impl Responder {
  let props = match require_auth(&jwt, &sessions).await {
    Ok(props) => props,
    Err(res) => return res,
  };

  match delete_comment_like(&comments, &ids.0, &ids.1, &props.uid).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(err) => build_api_err(500, err.to_string(), Some(err.to_string())),
  }
}

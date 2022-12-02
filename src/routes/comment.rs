use crate::{
  db::{
    comment_repository::CommentPool, follow_repository::FollowPool, post_repository::PostPool,
    session_repository::SessionPool,
  },
  helpers::auth::{query_auth, require_auth},
  helpers::core::{build_api_err, map_api_err},
  logic::comment::{create_comment, create_comment_like, delete_comment, delete_comment_like, get_comments},
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
    Err(err) => map_api_err(err),
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

  match get_comments(&comments, &post_id, &own_user_id, &query.page, &query.page_size).await {
    Ok(response) => HttpResponse::Ok().json(response),
    Err(err) => map_api_err(err),
  }
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
    Err(err) => map_api_err(err),
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
    Err(err) => map_api_err(err),
  }
}

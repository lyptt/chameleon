use actix_easy_multipart::tempfile::Tempfile;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::LogicErr;
use crate::{
  cdn::cdn_store::Cdn,
  model::{access_type::AccessType, post::Post, post_pub::PostPub},
};

#[derive(Debug, Deserialize)]
pub struct NewPostRequest {
  pub content_md: String,
  pub visibility: AccessType,
}

#[derive(Debug, Serialize)]
pub struct NewPostResponse {
  pub post_id: Uuid,
}

pub async fn get_user_posts(
  handle: &str,
  limit: i64,
  skip: i64,
  db: &Pool<Postgres>,
) -> Result<Vec<PostPub>, LogicErr> {
  PostPub::fetch_user_own_feed(handle, limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_user_posts_count(handle: &str, db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostPub::count_user_own_feed(handle, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_global_posts(limit: i64, skip: i64, db: &Pool<Postgres>) -> Result<Vec<PostPub>, LogicErr> {
  PostPub::fetch_global_federated_feed(limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_global_posts_count(db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostPub::count_global_federated_feed(&db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn create_post(db: &Pool<Postgres>, req: &NewPostRequest, user_id: &Uuid) -> Result<Uuid, LogicErr> {
  let content_html = markdown::to_html(&req.content_md);

  Post::create_post(user_id, &req.content_md, &content_html, &req.visibility, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn upload_post_file(
  db: &Pool<Postgres>,
  post_id: &Uuid,
  user_id: &Uuid,
  cdn: &Cdn,
  upload: &Tempfile,
) -> Result<Uuid, LogicErr> {
  if !Post::user_owns_post(&user_id, &post_id, &db).await {
    return Err(LogicErr::UnauthorizedError);
  }

  let file_name = format!("originals/{}", Uuid::new_v4());

  let path = match cdn.upload_file(&upload, &file_name).await {
    Ok(path) => path,
    Err(err) => return Err(err),
  };

  match Post::update_post_content_storage(&post_id, &path, &db).await {
    Ok(_) => {}
    Err(err) => return Err(LogicErr::DbError(err)),
  }

  // TODO: Create job in job queue
  let job_id = Uuid::new_v4();
  Ok(job_id)
}

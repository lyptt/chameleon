use actix_easy_multipart::tempfile::Tempfile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::LogicErr;
use crate::{
  cdn::cdn_store::Cdn,
  db::{job_repository::JobPool, post_repository::PostPool},
  helpers::api::map_db_err,
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    post_event::PostEvent,
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

#[derive(Debug, Deserialize)]
pub struct NewPostRequest {
  pub content_md: String,
  pub visibility: AccessType,
}

#[derive(Debug, Serialize)]
pub struct NewPostResponse {
  pub id: Uuid,
}

pub async fn get_user_posts(
  user_id: &Uuid,
  limit: i64,
  skip: i64,
  posts: &PostPool,
) -> Result<Vec<PostEvent>, LogicErr> {
  posts
    .fetch_user_own_feed(user_id, limit, skip)
    .await
    .map_err(map_db_err)
}

pub async fn get_post(post_id: &Uuid, user_id: &Option<Uuid>, posts: &PostPool) -> Result<Option<PostEvent>, LogicErr> {
  posts.fetch_post(post_id, user_id).await.map_err(map_db_err)
}

pub async fn get_user_posts_count(user_id: &Uuid, posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_user_own_feed(user_id).await.map_err(map_db_err)
}

pub async fn get_global_posts(limit: i64, skip: i64, posts: &PostPool) -> Result<Vec<PostEvent>, LogicErr> {
  posts.fetch_global_federated_feed(limit, skip).await.map_err(map_db_err)
}

pub async fn get_global_posts_count(posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_global_federated_feed().await.map_err(map_db_err)
}

pub async fn create_post(posts: &PostPool, req: &NewPostRequest, user_id: &Uuid) -> Result<Uuid, LogicErr> {
  let content_html = markdown::to_html(&req.content_md);

  posts
    .create_post(user_id, &req.content_md, &content_html, &req.visibility)
    .await
    .map_err(map_db_err)
}

pub async fn upload_post_file(
  posts: &PostPool,
  jobs: &JobPool,
  post_id: &Uuid,
  user_id: &Uuid,
  cdn: &Cdn,
  queue: &Queue,
  upload: &Tempfile,
) -> Result<Uuid, LogicErr> {
  if !posts.user_owns_post(user_id, post_id).await {
    return Err(LogicErr::UnauthorizedError);
  }

  let file_name = format!("media/{}/or/{}", user_id, Uuid::new_v4());

  let path = match cdn.upload_tmp_file(upload, &file_name).await {
    Ok(path) => path,
    Err(err) => return Err(err),
  };

  match posts.update_post_content_storage(post_id, &path).await {
    Ok(_) => {}
    Err(err) => return Err(map_db_err(err)),
  }

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(*user_id),
      status: JobStatus::NotStarted,
      record_id: Some(*post_id),
      associated_record_id: None,
    })
    .await
    .map_err(map_db_err)?;

  let job = QueueJob {
    job_id,
    job_type: QueueJobType::ConvertNewPostImages,
  };

  match queue.send_job(job).await {
    Ok(_) => Ok(job_id),
    Err(err) => Err(err),
  }
}

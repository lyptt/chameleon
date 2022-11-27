use actix_easy_multipart::tempfile::Tempfile;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::LogicErr;
use crate::{
  cdn::cdn_store::Cdn,
  helpers::api::map_db_err,
  model::{
    access_type::AccessType,
    job::{Job, JobStatus, NewJob},
    post::Post,
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
  db: &Pool<Postgres>,
) -> Result<Vec<PostEvent>, LogicErr> {
  PostEvent::fetch_user_own_feed(user_id, limit, skip, db)
    .await
    .map_err(map_db_err)
}

pub async fn get_post(
  post_id: &Uuid,
  user_id: &Option<Uuid>,
  db: &Pool<Postgres>,
) -> Result<Option<PostEvent>, LogicErr> {
  PostEvent::fetch_post(post_id, user_id, db).await.map_err(map_db_err)
}

pub async fn get_user_posts_count(user_id: &Uuid, db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostEvent::count_user_own_feed(user_id, db).await.map_err(map_db_err)
}

pub async fn get_global_posts(limit: i64, skip: i64, db: &Pool<Postgres>) -> Result<Vec<PostEvent>, LogicErr> {
  PostEvent::fetch_global_federated_feed(limit, skip, db)
    .await
    .map_err(map_db_err)
}

pub async fn get_global_posts_count(db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostEvent::count_global_federated_feed(db).await.map_err(map_db_err)
}

pub async fn create_post(db: &Pool<Postgres>, req: &NewPostRequest, user_id: &Uuid) -> Result<Uuid, LogicErr> {
  let content_html = markdown::to_html(&req.content_md);

  Post::create_post(user_id, &req.content_md, &content_html, &req.visibility, db)
    .await
    .map_err(map_db_err)
}

pub async fn upload_post_file(
  db: &Pool<Postgres>,
  post_id: &Uuid,
  user_id: &Uuid,
  cdn: &Cdn,
  queue: &Queue,
  upload: &Tempfile,
) -> Result<Uuid, LogicErr> {
  if !Post::user_owns_post(user_id, post_id, db).await {
    return Err(LogicErr::UnauthorizedError);
  }

  let file_name = format!("media/{}/or/{}", user_id, Uuid::new_v4());

  let path = match cdn.upload_tmp_file(upload, &file_name).await {
    Ok(path) => path,
    Err(err) => return Err(err),
  };

  match Post::update_post_content_storage(post_id, &path, db).await {
    Ok(_) => {}
    Err(err) => return Err(map_db_err(err)),
  }

  let job_id = Uuid::new_v4();
  Job::create(
    NewJob {
      job_id,
      created_by_id: Some(*user_id),
      status: JobStatus::NotStarted,
      completion_record_id: Some(*post_id),
    },
    db,
  )
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

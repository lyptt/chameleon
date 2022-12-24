use actix_easy_multipart::tempfile::Tempfile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::LogicErr;
use crate::{
  cdn::cdn_store::Cdn,
  db::{job_repository::JobPool, post_repository::PostPool},
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    post_event::PostEvent,
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
  posts.fetch_user_own_feed(user_id, limit, skip).await
}

pub async fn get_post(post_id: &Uuid, user_id: &Option<Uuid>, posts: &PostPool) -> Result<Option<PostEvent>, LogicErr> {
  posts.fetch_post(post_id, user_id).await
}

pub async fn get_user_posts_count(user_id: &Uuid, posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_user_own_feed(user_id).await
}

pub async fn get_global_posts(limit: i64, skip: i64, posts: &PostPool) -> Result<Vec<PostEvent>, LogicErr> {
  posts.fetch_global_federated_feed(limit, skip).await
}

pub async fn get_global_posts_count(posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_global_federated_feed().await
}

pub async fn create_post(posts: &PostPool, req: &NewPostRequest, user_id: &Uuid) -> Result<Uuid, LogicErr> {
  let content_html = markdown::to_html(&req.content_md);

  posts
    .create_post(user_id, &req.content_md, &content_html, &req.visibility)
    .await
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
    Err(err) => return Err(err),
  }

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(*user_id),
      status: JobStatus::NotStarted,
      record_id: Some(*post_id),
      associated_record_id: None,
    })
    .await?;

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::ConvertNewPostImages)
    .build();

  match queue.send_job(job).await {
    Ok(_) => Ok(job_id),
    Err(err) => Err(err),
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use actix_easy_multipart::tempfile::Tempfile;
  use chrono::Utc;
  use mockall::predicate::{str::*, *};
  use tempfile::NamedTempFile;
  use uuid::Uuid;

  use crate::{
    cdn::cdn_store::{Cdn, MockCdnStore},
    db::{
      job_repository::{JobPool, MockJobRepo},
      post_repository::{MockPostRepo, PostPool},
    },
    logic::{
      post::{
        create_post, get_global_posts, get_global_posts_count, get_post, get_user_posts, get_user_posts_count,
        upload_post_file, NewPostRequest,
      },
      LogicErr,
    },
    model::{access_type::AccessType, event_type::EventType, post_event::PostEvent},
    work_queue::queue::{MockQueueBackend, Queue},
  };

  #[async_std::test]
  async fn get_user_posts_db_err_passthrough() {
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_user_own_feed()
      .times(1)
      .with(eq(user_id), eq(1), eq(2))
      .returning(|_, _, _| Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      get_user_posts(&user_id, 1, 2, &posts).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn get_user_posts_succeeds() {
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_user_own_feed()
      .times(1)
      .with(eq(user_id), eq(1), eq(2))
      .returning(|_, _, _| Ok(vec![]));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(get_user_posts(&user_id, 1, 2, &posts).await, Ok(vec![]));
  }

  #[async_std::test]
  async fn get_user_post_db_err_passthrough() {
    let user_id = Some(Uuid::new_v4());
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_post()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      get_post(&post_id, &user_id, &posts).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn get_user_post_succeeds() {
    let user_id = Some(Uuid::new_v4());
    let post_id = Uuid::new_v4();

    let post = PostEvent {
      event_type: EventType::Post,
      post_id,
      uri: "a".to_string(),
      content_md: "a".to_string(),
      content_html: "a".to_string(),
      content_image_uri_small: None,
      content_image_uri_medium: None,
      content_image_uri_large: None,
      content_width_small: None,
      content_width_medium: None,
      content_width_large: None,
      content_height_small: None,
      content_height_medium: None,
      content_height_large: None,
      content_type_small: None,
      content_type_medium: None,
      content_type_large: None,
      visibility: AccessType::PublicFederated,
      created_at: Utc::now(),
      updated_at: Utc::now(),
      content_blurhash: None,
      user_id: user_id.unwrap(),
      user_handle: "a".to_string(),
      user_fediverse_id: "a".to_string(),
      user_fediverse_uri: "a".to_string(),
      user_avatar_url: None,
      event_user_handle: "a".to_string(),
      event_user_fediverse_id: "a".to_string(),
      event_user_fediverse_uri: "a".to_string(),
      event_user_avatar_url: None,
      likes: 1,
      liked: Some(false),
      comments: 1,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_post()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .return_const(Ok(Some(post.clone())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(get_post(&post_id, &user_id, &posts).await, Ok(Some(post)));
  }

  #[async_std::test]
  async fn get_user_post_count_db_err_passthrough() {
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_count_user_own_feed()
      .times(1)
      .with(eq(user_id))
      .return_const(Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      get_user_posts_count(&user_id, &posts).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn get_user_post_count_succeeds() {
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_count_user_own_feed()
      .times(1)
      .with(eq(user_id))
      .return_const(Ok(123));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(get_user_posts_count(&user_id, &posts).await, Ok(123));
  }

  #[async_std::test]
  async fn get_global_posts_db_err_passthrough() {
    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_global_federated_feed()
      .times(1)
      .with(eq(1), eq(2))
      .return_const(Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      get_global_posts(1, 2, &posts).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn get_global_posts_succeeds() {
    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_fetch_global_federated_feed()
      .times(1)
      .with(eq(1), eq(2))
      .return_const(Ok(vec![]));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(get_global_posts(1, 2, &posts).await, Ok(vec![]));
  }

  #[async_std::test]
  async fn get_global_posts_count_db_err_passthrough() {
    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_count_global_federated_feed()
      .times(1)
      .return_const(Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      get_global_posts_count(&posts).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn get_global_posts_count_succeeds() {
    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_count_global_federated_feed()
      .times(1)
      .return_const(Ok(123));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(get_global_posts_count(&posts).await, Ok(123));
  }

  #[async_std::test]
  async fn create_post_db_err_passthrough() {
    let user_id = Uuid::new_v4();

    let new_post = NewPostRequest {
      content_md: "hello\n**world**!".to_string(),
      visibility: AccessType::PublicFederated,
    };
    let content_md_eq = new_post.content_md.clone();
    let visibility_eq = new_post.visibility.clone();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_create_post()
      .with(eq(user_id), contains(content_md_eq), always(), eq(visibility_eq))
      .times(1)
      .return_const(Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      create_post(&posts, &new_post, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn create_post_succeeds() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let new_post = NewPostRequest {
      content_md: "hello\n**world**!".to_string(),
      visibility: AccessType::PublicFederated,
    };
    let content_md_eq = new_post.content_md.clone();
    let visibility_eq = new_post.visibility.clone();

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_create_post()
      .with(eq(user_id), contains(content_md_eq), always(), eq(visibility_eq))
      .times(1)
      .return_const(Ok(post_id));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(create_post(&posts, &new_post, &user_id).await, Ok(post_id));
  }

  #[async_std::test]
  async fn upload_post_file_fails_invalid_post() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(false);

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let cdn = Cdn::new_inner(Box::new(MockCdnStore::new()));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn upload_post_file_fails_upload_err() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(true);

    let mut cdn_store = MockCdnStore::new();
    cdn_store
      .expect_upload_tmp_file()
      .with(always(), starts_with("media/"))
      .return_const(Err(LogicErr::InternalError("Upload failed".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Err(LogicErr::InternalError("Upload failed".to_string()))
    );
  }

  #[async_std::test]
  async fn upload_post_file_fails_update_post_err() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(true);

    post_repo
      .expect_update_post_content_storage()
      .with(eq(post_id), starts_with("/a/b/c"))
      .times(1)
      .return_const(Err(LogicErr::MissingRecord));

    let mut cdn_store = MockCdnStore::new();
    cdn_store
      .expect_upload_tmp_file()
      .with(always(), starts_with("media/"))
      .return_const(Ok("/a/b/c".to_string()));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn upload_post_file_fails_create_job_err() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(true);

    post_repo
      .expect_update_post_content_storage()
      .with(eq(post_id), starts_with("/a/b/c"))
      .times(1)
      .return_const(Ok(()));

    let mut cdn_store = MockCdnStore::new();
    cdn_store
      .expect_upload_tmp_file()
      .with(always(), starts_with("media/"))
      .return_const(Ok("/a/b/c".to_string()));

    let mut job_repo = MockJobRepo::new();
    job_repo
      .expect_create()
      .with(always())
      .return_const(Err(LogicErr::DbError("Insert failed".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(job_repo);
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Err(LogicErr::DbError("Insert failed".to_string()))
    );
  }

  #[async_std::test]
  async fn upload_post_file_fails_register_job_err() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(true);

    post_repo
      .expect_update_post_content_storage()
      .with(eq(post_id), starts_with("/a/b/c"))
      .times(1)
      .return_const(Ok(()));

    let mut cdn_store = MockCdnStore::new();
    cdn_store
      .expect_upload_tmp_file()
      .with(always(), starts_with("media/"))
      .return_const(Ok("/a/b/c".to_string()));

    let mut job_repo = MockJobRepo::new();
    job_repo.expect_create().with(always()).return_const(Ok(job_id));

    let mut queue_backend = MockQueueBackend::new();
    queue_backend
      .expect_send_job()
      .with(always())
      .return_const(Err(LogicErr::InternalError("Job failed".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(job_repo);
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(queue_backend));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Err(LogicErr::InternalError("Job failed".to_string()))
    );
  }

  #[async_std::test]
  async fn upload_post_file_succeeds() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();
    let tempfile = Tempfile {
      file: NamedTempFile::new().unwrap(),
      content_type: None,
      file_name: None,
      size: 0,
    };

    let mut post_repo = MockPostRepo::new();
    post_repo
      .expect_user_owns_post()
      .with(eq(user_id), eq(post_id))
      .times(1)
      .return_const(true);

    post_repo
      .expect_update_post_content_storage()
      .with(eq(post_id), starts_with("/a/b/c"))
      .times(1)
      .return_const(Ok(()));

    let mut cdn_store = MockCdnStore::new();
    cdn_store
      .expect_upload_tmp_file()
      .with(always(), starts_with("media/"))
      .return_const(Ok("/a/b/c".to_string()));

    let mut job_repo = MockJobRepo::new();
    job_repo.expect_create().with(always()).return_const(Ok(job_id));

    let mut queue_backend = MockQueueBackend::new();
    queue_backend.expect_send_job().with(always()).return_const(Ok(()));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(job_repo);
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(queue_backend));

    assert_eq!(
      upload_post_file(&posts, &jobs, &post_id, &user_id, &cdn, &queue, &tempfile).await,
      Ok(job_id)
    );
  }
}

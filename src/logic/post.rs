use actix_easy_multipart::tempfile::Tempfile;
use chrono::Utc;
use futures_util::future::join_all;
use serde::{Deserialize, Serialize};
use std::{pin::Pin, str::FromStr};
use uuid::Uuid;

use super::LogicErr;
use crate::{
  cdn::cdn_store::Cdn,
  db::{job_repository::JobPool, post_attachment_repository::PostAttachmentPool, post_repository::PostPool},
  helpers::api::{map_db_err, map_ext_err},
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    post_attachment::PostAttachment,
    post_event::PostEvent,
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct NewPostRequest {
  pub title: Option<String>,
  pub content_md: String,
  pub visibility: AccessType,
  pub orbit_id: Option<Uuid>,
  pub attachment_count: i64,
}

#[derive(Debug, Serialize)]
pub struct NewPostResponse {
  pub id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatePostResult {
  WaitingForImages(Uuid),
  JobQueued(Uuid),
}

pub async fn get_user_posts(
  user_id: &Uuid,
  limit: i64,
  skip: i64,
  posts: &PostPool,
) -> Result<Vec<PostEvent>, LogicErr> {
  posts.fetch_user_own_feed(user_id, limit, skip).await
}

pub async fn get_user_friends_posts(
  user_id: &Uuid,
  limit: i64,
  skip: i64,
  posts: &PostPool,
) -> Result<Vec<PostEvent>, LogicErr> {
  posts.fetch_user_friends_feed(user_id, limit, skip).await
}

pub async fn get_post(post_id: &Uuid, user_id: &Option<Uuid>, posts: &PostPool) -> Result<Option<PostEvent>, LogicErr> {
  posts.fetch_post(post_id, user_id).await
}

pub async fn get_user_posts_count(user_id: &Uuid, posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_user_own_feed(user_id).await
}

pub async fn get_user_friends_posts_count(user_id: &Uuid, posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_user_friends_feed(user_id).await
}

pub async fn get_global_posts(limit: i64, skip: i64, posts: &PostPool) -> Result<Vec<PostEvent>, LogicErr> {
  posts.fetch_global_federated_feed(limit, skip).await
}

pub async fn get_global_posts_count(posts: &PostPool) -> Result<i64, LogicErr> {
  posts.count_global_federated_feed().await
}

pub async fn create_post(
  posts: &PostPool,
  jobs: &JobPool,
  queue: &Queue,
  req: &NewPostRequest,
  user_id: &Uuid,
) -> Result<CreatePostResult, LogicErr> {
  let content_html = markdown::to_html(&req.content_md);

  let post_id = posts
    .create_post(
      user_id,
      &req.content_md,
      &content_html,
      &req.visibility,
      &req.orbit_id,
      &req.title,
    )
    .await?;

  if req.attachment_count > 0 {
    return Ok(CreatePostResult::WaitingForImages(post_id));
  }

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(user_id.to_owned()),
      status: JobStatus::NotStarted,
      record_id: Some(post_id.to_owned()),
      associated_record_id: None,
    })
    .await
    .map_err(map_db_err)?;

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::CreatePostEvents)
    .build();

  match queue.send_job(job).await {
    Ok(_) => Ok(CreatePostResult::JobQueued(job_id)),
    Err(err) => Err(err),
  }
}

async fn upload_post_file(
  post_attachments: &PostAttachmentPool,
  post_id: &Uuid,
  user_id: &Uuid,
  cdn: &Cdn,
  upload: &Tempfile,
) -> Result<(), LogicErr> {
  let file_name = match upload.file_name.to_owned() {
    Some(name) => name,
    None => return Err(LogicErr::InvalidData),
  };

  let content_type = match mime_guess::from_path(&file_name).first() {
    Some(m) => m.to_string(),
    None => return Err(LogicErr::InternalError("Unsupported file type".to_string())),
  };

  let mime = mime::Mime::from_str(&content_type).map_err(map_ext_err)?;
  if mime != mime::IMAGE_PNG && mime != mime::IMAGE_JPEG {
    return Err(LogicErr::InvalidData);
  }

  let metadata = immeta::load_from_file(upload.file.path()).map_err(map_ext_err)?;
  let dimens = metadata.dimensions();

  let file_name = format!("media/{}/or/{}", user_id, Uuid::new_v4());

  let path = match cdn.upload_tmp_file(upload, &content_type, &file_name).await {
    Ok(path) => path,
    Err(err) => return Err(err),
  };

  let attachment = PostAttachment {
    attachment_id: Uuid::new_v4(),
    user_id: *user_id,
    post_id: *post_id,
    uri: Some(format!("/{}", path)),
    width: dimens.width.try_into().unwrap_or_default(),
    height: dimens.height.try_into().unwrap_or_default(),
    content_type: Some(content_type),
    storage_ref: Some(path),
    blurhash: None,
    created_at: Utc::now(),
  };

  post_attachments.create_attachment_from(attachment).await?;
  Ok(())
}

pub async fn upload_post_files(
  posts: &PostPool,
  jobs: &JobPool,
  post_attachments: &PostAttachmentPool,
  post_id: &Uuid,
  user_id: &Uuid,
  cdn: &Cdn,
  queue: &Queue,
  uploads: &[Tempfile],
) -> Result<Uuid, LogicErr> {
  if !posts.user_owns_post(user_id, post_id).await {
    return Err(LogicErr::UnauthorizedError);
  }

  // This type is complex, yes, but also unavoidable due to the types we have to work with here
  #[allow(clippy::type_complexity)]
  let mut futures: Vec<
    Pin<Box<dyn futures_util::Future<Output = std::result::Result<(), LogicErr>> + std::marker::Send>>,
  > = vec![];

  for upload in uploads {
    futures.push(Box::pin(upload_post_file(
      post_attachments,
      post_id,
      user_id,
      cdn,
      upload,
    )));
  }

  let results = join_all(futures).await;
  let results_len = results.len();
  let mut err_count = 0;

  for result in results {
    if let Err(err) = result {
      log::error!("Failed to upload attachment: {}", err);
      err_count += 1;
    }
  }

  if err_count == results_len {
    return Err(LogicErr::InternalError("Failed to process all attachments".to_owned()));
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

  queue.send_job(job).await?;

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(user_id.to_owned()),
      status: JobStatus::NotStarted,
      record_id: Some(post_id.to_owned()),
      associated_record_id: None,
    })
    .await
    .map_err(map_db_err)?;

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::CreatePostEvents)
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
      post_attachment_repository::{MockPostAttachmentRepo, PostAttachmentPool},
      post_repository::{MockPostRepo, PostPool},
    },
    logic::{
      post::{
        create_post, get_global_posts, get_global_posts_count, get_post, get_user_posts, get_user_posts_count,
        upload_post_files, NewPostRequest,
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
      title: None,
      content_md: "a".to_string(),
      content_html: "a".to_string(),
      visibility: AccessType::PublicFederated,
      created_at: Utc::now(),
      updated_at: Utc::now(),
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
      attachments: vec![],
      orbit_id: None,
      orbit_name: None,
      orbit_uri: None,
      orbit_fediverse_uri: None,
      orbit_avatar_uri: None,
      orbit_shortcode: None,
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
      orbit_id: None,
      attachment_count: 0,
      title: None,
    };
    let content_md_eq = new_post.content_md.clone();
    let visibility_eq = new_post.visibility.clone();

    let mut post_repo = MockPostRepo::new();
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));
    post_repo
      .expect_create_post()
      .with(
        eq(user_id),
        contains(content_md_eq),
        always(),
        eq(visibility_eq),
        eq(None),
        eq(None),
      )
      .times(1)
      .return_const(Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);

    assert_eq!(
      create_post(&posts, &jobs, &queue, &new_post, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn create_post_succeeds() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();

    let new_post = NewPostRequest {
      content_md: "hello\n**world**!".to_string(),
      visibility: AccessType::PublicFederated,
      orbit_id: None,
      attachment_count: 0,
      title: None,
    };
    let content_md_eq = new_post.content_md.clone();
    let visibility_eq = new_post.visibility.clone();

    let mut post_repo = MockPostRepo::new();
    let mut job_repo = MockJobRepo::new();
    let mut queue_be = MockQueueBackend::new();

    post_repo
      .expect_create_post()
      .with(
        eq(user_id),
        contains(content_md_eq),
        always(),
        eq(visibility_eq),
        eq(None),
        eq(None),
      )
      .times(1)
      .return_const(Ok(post_id));

    job_repo
      .expect_create()
      .with(always())
      .times(1)
      .return_const(Ok(job_id));

    queue_be.expect_send_job().with(always()).times(1).return_const(Ok(()));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(job_repo);
    let queue = Queue::new_inner(Box::new(queue_be));

    assert!(create_post(&posts, &jobs, &queue, &new_post, &user_id).await.is_ok(),);
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
    let post_attachments: PostAttachmentPool = Arc::new(MockPostAttachmentRepo::new());
    let cdn = Cdn::new_inner(Box::new(MockCdnStore::new()));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_files(
        &posts,
        &jobs,
        &post_attachments,
        &post_id,
        &user_id,
        &cdn,
        &queue,
        &[tempfile]
      )
      .await,
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
      .with(always(), always(), starts_with("media/"))
      .return_const(Err(LogicErr::InternalError("Upload failed".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let post_attachments: PostAttachmentPool = Arc::new(MockPostAttachmentRepo::new());
    let cdn = Cdn::new_inner(Box::new(cdn_store));
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      upload_post_files(
        &posts,
        &jobs,
        &post_attachments,
        &post_id,
        &user_id,
        &cdn,
        &queue,
        &[tempfile]
      )
      .await,
      Err(LogicErr::InternalError("Failed to process all attachments".to_string()))
    );
  }
}

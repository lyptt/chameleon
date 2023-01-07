use uuid::Uuid;

use crate::{
  db::{follow_repository::FollowPool, job_repository::JobPool, user_repository::UserPool},
  federation::activitypub::{FederateExtAction, FederateExtActorRef},
  helpers::api::map_db_err,
  model::{
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

use super::LogicErr;

pub async fn create_follow(
  users: &UserPool,
  follows: &FollowPool,
  jobs: &JobPool,
  queue: &Queue,
  following_user_handle: &str,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  let following_user = match users.fetch_by_handle(following_user_handle).await? {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  let following_user_id = following_user.user_id;

  if following_user.is_external {
    let job_id = jobs
      .create(NewJob {
        created_by_id: Some(*user_id),
        status: JobStatus::NotStarted,
        record_id: Some(following_user_id),
        associated_record_id: None,
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::FederateActivityPubExt)
      .context(vec![user_id.to_string()])
      .activitypub_federate_ext_action(FederateExtAction::FollowProfile)
      .activitypub_federate_ext_dest_actor(FederateExtActorRef::Person(following_user_id))
      .build();

    queue.send_job(job).await?;
  }

  follows.create_follow(user_id, &following_user_id).await?;
  Ok(())
}

pub async fn delete_follow(
  users: &UserPool,
  follows: &FollowPool,
  jobs: &JobPool,
  queue: &Queue,
  following_user_handle: &str,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  let following_user = match users.fetch_by_handle(following_user_handle).await? {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  let following_user_id = following_user.user_id;

  if following_user.is_external {
    let job_id = jobs
      .create(NewJob {
        created_by_id: Some(*user_id),
        status: JobStatus::NotStarted,
        record_id: Some(following_user_id),
        associated_record_id: None,
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::FederateActivityPubExt)
      .context(vec![user_id.to_string()])
      .activitypub_federate_ext_action(FederateExtAction::UnfollowProfile)
      .activitypub_federate_ext_dest_actor(FederateExtActorRef::Person(following_user_id))
      .build();

    queue.send_job(job).await?;
  }

  follows.delete_follow(user_id, &following_user_id).await
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use chrono::Utc;
  use mockall::predicate::*;
  use uuid::Uuid;

  use crate::{
    db::{
      follow_repository::{FollowPool, MockFollowRepo},
      job_repository::{JobPool, MockJobRepo},
      user_repository::{MockUserRepo, UserPool},
    },
    logic::{
      follow::{create_follow, delete_follow},
      LogicErr,
    },
    model::user::User,
    work_queue::queue::{MockQueueBackend, Queue},
  };

  #[async_std::test]
  async fn test_create_follow_rejects_for_missing_following_user() {
    let user_id = Uuid::new_v4();
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .return_const(Ok(None));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      create_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_follow_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let following_user = User {
      user_id: following_user_id,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: "a".to_string(),
      fediverse_uri: "d".to_string(),
      avatar_url: None,
      email: Some("b".to_string()),
      password_hash: Some("c".to_string()),
      is_external: false,
      url_1: None,
      url_2: None,
      url_3: None,
      url_4: None,
      url_5: None,
      url_1_title: None,
      url_2_title: None,
      url_3_title: None,
      url_4_title: None,
      url_5_title: None,
      intro_md: None,
      intro_html: None,
      private_key: "d".to_string(),
      public_key: "e".to_string(),
      ext_apub_followers_uri: None,
      ext_apub_following_uri: None,
      ext_apub_inbox_uri: None,
      ext_apub_outbox_uri: None,
      created_at: Utc::now(),
    };

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .return_const(Ok(Some(following_user)));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_create_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Err(LogicErr::DbError("Boop".to_string())));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      create_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_follow_succeeds() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let follow_id = Uuid::new_v4();
    let following_user_handle = "user_handle".to_string();

    let following_user = User {
      user_id: following_user_id,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: "a".to_string(),
      fediverse_uri: "d".to_string(),
      avatar_url: None,
      email: Some("b".to_string()),
      password_hash: Some("c".to_string()),
      is_external: false,
      url_1: None,
      url_2: None,
      url_3: None,
      url_4: None,
      url_5: None,
      url_1_title: None,
      url_2_title: None,
      url_3_title: None,
      url_4_title: None,
      url_5_title: None,
      intro_md: None,
      intro_html: None,
      private_key: "d".to_string(),
      public_key: "e".to_string(),
      ext_apub_followers_uri: None,
      ext_apub_following_uri: None,
      ext_apub_inbox_uri: None,
      ext_apub_outbox_uri: None,
      created_at: Utc::now(),
    };

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .return_const(Ok(Some(following_user)));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_create_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(move |_, _| Ok(follow_id));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      create_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Ok(())
    );
  }

  #[async_std::test]
  async fn test_delete_follow_rejects_for_missing_user() {
    let user_id = Uuid::new_v4();
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(|_| Ok(None));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      delete_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_delete_follow_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let following_user = User {
      user_id: following_user_id,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: "a".to_string(),
      fediverse_uri: "d".to_string(),
      avatar_url: None,
      email: Some("b".to_string()),
      password_hash: Some("c".to_string()),
      is_external: false,
      url_1: None,
      url_2: None,
      url_3: None,
      url_4: None,
      url_5: None,
      url_1_title: None,
      url_2_title: None,
      url_3_title: None,
      url_4_title: None,
      url_5_title: None,
      intro_md: None,
      intro_html: None,
      private_key: "d".to_string(),
      public_key: "e".to_string(),
      ext_apub_followers_uri: None,
      ext_apub_following_uri: None,
      ext_apub_inbox_uri: None,
      ext_apub_outbox_uri: None,
      created_at: Utc::now(),
    };

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .return_const(Ok(Some(following_user)));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_delete_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Err(LogicErr::DbError("Boop".to_string())));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      delete_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_delete_follow_succeeds() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let following_user = User {
      user_id: following_user_id,
      fediverse_id: "user@127.0.0.1:8000".to_string(),
      handle: "a".to_string(),
      fediverse_uri: "d".to_string(),
      avatar_url: None,
      email: Some("b".to_string()),
      password_hash: Some("c".to_string()),
      is_external: false,
      url_1: None,
      url_2: None,
      url_3: None,
      url_4: None,
      url_5: None,
      url_1_title: None,
      url_2_title: None,
      url_3_title: None,
      url_4_title: None,
      url_5_title: None,
      intro_md: None,
      intro_html: None,
      private_key: "d".to_string(),
      public_key: "e".to_string(),
      ext_apub_followers_uri: None,
      ext_apub_following_uri: None,
      ext_apub_inbox_uri: None,
      ext_apub_outbox_uri: None,
      created_at: Utc::now(),
    };

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .return_const(Ok(Some(following_user)));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_delete_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Ok(()));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let jobs: JobPool = Arc::new(MockJobRepo::new());
    let queue = Queue::new_inner(Box::new(MockQueueBackend::new()));

    assert_eq!(
      delete_follow(&users, &follows, &jobs, &queue, &following_user_handle, &user_id).await,
      Ok(())
    );
  }
}

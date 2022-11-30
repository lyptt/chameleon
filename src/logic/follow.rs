use uuid::Uuid;

use crate::db::{follow_repository::FollowPool, user_repository::UserPool};

use super::LogicErr;

pub async fn create_follow(
  users: &UserPool,
  follows: &FollowPool,
  following_user_handle: &str,
  user_id: &Uuid,
) -> Result<Uuid, LogicErr> {
  let following_user_id = match users.fetch_id_by_handle(following_user_handle).await {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  follows.create_follow(user_id, &following_user_id).await
}

pub async fn delete_follow(
  users: &UserPool,
  follows: &FollowPool,
  following_user_handle: &str,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  let following_user_id = match users.fetch_id_by_handle(following_user_handle).await {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  follows.delete_follow(user_id, &following_user_id).await
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use mockall::predicate::*;
  use uuid::Uuid;

  use crate::{
    db::{
      follow_repository::{FollowPool, MockFollowRepo},
      user_repository::{MockUserRepo, UserPool},
    },
    logic::{
      follow::{create_follow, delete_follow},
      LogicErr,
    },
  };

  #[async_std::test]
  async fn test_create_follow_rejects_for_missing_user() {
    let user_id = Uuid::new_v4();
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(|_| None);

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());

    assert_eq!(
      create_follow(&users, &follows, &following_user_handle, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_follow_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(move |_| Some(following_user_id));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_create_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Err(LogicErr::DbError("Boop".to_string())));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);

    assert_eq!(
      create_follow(&users, &follows, &following_user_handle, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_follow_succeeds() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let follow_id = Uuid::new_v4();
    let follow_id_eq = follow_id;
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(move |_| Some(following_user_id));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_create_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(move |_, _| Ok(follow_id));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);

    assert_eq!(
      create_follow(&users, &follows, &following_user_handle, &user_id).await,
      Ok(follow_id_eq)
    );
  }

  #[async_std::test]
  async fn test_delete_follow_rejects_for_missing_user() {
    let user_id = Uuid::new_v4();
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(|_| None);

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());

    assert_eq!(
      delete_follow(&users, &follows, &following_user_handle, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_delete_follow_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(move |_| Some(following_user_id));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_delete_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Err(LogicErr::DbError("Boop".to_string())));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);

    assert_eq!(
      delete_follow(&users, &follows, &following_user_handle, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_delete_follow_succeeds() {
    let user_id = Uuid::new_v4();
    let following_user_id = Uuid::new_v4();
    let following_user_id_eq = following_user_id;
    let following_user_handle = "user_handle".to_string();

    let mut user_repo = MockUserRepo::new();

    user_repo
      .expect_fetch_id_by_handle()
      .times(1)
      .with(eq("user_handle"))
      .returning(move |_| Some(following_user_id));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_delete_follow()
      .times(1)
      .with(eq(user_id), eq(following_user_id_eq))
      .returning(|_, _| Ok(()));

    let users: UserPool = Arc::new(user_repo);
    let follows: FollowPool = Arc::new(follow_repo);

    assert_eq!(
      delete_follow(&users, &follows, &following_user_handle, &user_id).await,
      Ok(())
    );
  }
}

use uuid::Uuid;

use crate::{
  db::{follow_repository::FollowPool, like_repository::LikePool, post_repository::PostPool},
  model::access_type::AccessType,
};

use super::LogicErr;

pub async fn create_like(
  posts: &PostPool,
  follows: &FollowPool,
  likes: &LikePool,
  post_id: &Uuid,
  user_id: &Uuid,
) -> Result<Uuid, LogicErr> {
  let visibility = match posts.fetch_visibility_by_id(post_id).await {
    Some(visibility) => visibility,
    None => return Err(LogicErr::MissingRecord),
  };

  let owner_id = match posts.fetch_owner_by_id(post_id).await {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  // If the commenting user doesn't own the post and the post isn't publicly available, don't let the user like the post
  if (visibility == AccessType::Private || visibility == AccessType::Shadow) && &owner_id != user_id {
    return Err(LogicErr::UnauthorizedError);
  }

  // If the post is only available to the author's followers and the user isn't a follower of the author, don't let the
  // user like the post
  if visibility == AccessType::FollowersOnly && !follows.user_follows_poster(post_id, user_id).await {
    return Err(LogicErr::MissingRecord);
  }

  likes.create_like(user_id, post_id).await
}

pub async fn delete_like(likes: &LikePool, post_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
  likes.delete_like(user_id, post_id).await
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use mockall::predicate::*;
  use uuid::Uuid;

  use crate::{
    db::{
      follow_repository::{FollowPool, MockFollowRepo},
      like_repository::{LikePool, MockLikeRepo},
      post_repository::{MockPostRepo, PostPool},
    },
    logic::{
      like::{create_like, delete_like},
      LogicErr,
    },
    model::access_type::AccessType,
  };

  #[async_std::test]
  async fn test_create_like_rejects_for_missing_post() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| None);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let likes: LikePool = Arc::new(MockLikeRepo::new());

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_like_rejects_for_missing_user() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::PublicFederated));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| None);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let likes: LikePool = Arc::new(MockLikeRepo::new());

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_like_rejects_for_ineligible_user() {
    let user_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::Private));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(move |_| Some(owner_id));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let likes: LikePool = Arc::new(MockLikeRepo::new());

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_create_like_rejects_for_anonymous_user() {
    let user_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::FollowersOnly));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(move |_| Some(owner_id));

    let mut follow_repo = MockFollowRepo::new();
    follow_repo
      .expect_user_follows_poster()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| false);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let likes: LikePool = Arc::new(MockLikeRepo::new());

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_like_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::FollowersOnly));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(move |_| Some(owner_id));

    let mut follow_repo = MockFollowRepo::new();
    follow_repo
      .expect_user_follows_poster()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| true);

    let mut like_repo = MockLikeRepo::new();
    like_repo
      .expect_create_like()
      .times(1)
      .with(eq(user_id), eq(post_id))
      .returning(move |_, _| Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let likes: LikePool = Arc::new(like_repo);

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_like_succeeds() {
    let user_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();
    let like_id = Uuid::new_v4();
    let like_id_eq = like_id;

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::FollowersOnly));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(move |_| Some(owner_id));

    let mut follow_repo = MockFollowRepo::new();
    follow_repo
      .expect_user_follows_poster()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| true);

    let mut like_repo = MockLikeRepo::new();
    like_repo
      .expect_create_like()
      .times(1)
      .with(eq(user_id), eq(post_id))
      .returning(move |_, _| Ok(like_id));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let likes: LikePool = Arc::new(like_repo);

    assert_eq!(
      create_like(&posts, &follows, &likes, &post_id, &user_id).await,
      Ok(like_id_eq)
    );
  }

  #[async_std::test]
  async fn test_delete_like_db_err_passthrough() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut like_repo = MockLikeRepo::new();
    like_repo
      .expect_delete_like()
      .times(1)
      .with(eq(user_id), eq(post_id))
      .returning(move |_, _| Err(LogicErr::DbError("Boop".to_string())));

    let likes: LikePool = Arc::new(like_repo);

    assert_eq!(
      delete_like(&likes, &post_id, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_delete_like_succeeds() {
    let user_id = Uuid::new_v4();
    let post_id = Uuid::new_v4();

    let mut like_repo = MockLikeRepo::new();
    like_repo
      .expect_delete_like()
      .times(1)
      .with(eq(user_id), eq(post_id))
      .returning(move |_, _| Ok(()));

    let likes: LikePool = Arc::new(like_repo);

    assert_eq!(delete_like(&likes, &post_id, &user_id).await, Ok(()));
  }
}

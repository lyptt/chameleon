use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible, document::ActivityPubDocument,
    helpers::create_activitypub_ordered_collection_page,
  },
  db::{comment_repository::CommentPool, follow_repository::FollowPool, post_repository::PostPool},
  helpers::math::div_up,
  model::{access_type::AccessType, comment_pub::CommentPub, response::ListResponse},
  settings::SETTINGS,
};

use super::LogicErr;

pub async fn create_comment(
  posts: &PostPool,
  follows: &FollowPool,
  comments: &CommentPool,
  post_id: &Uuid,
  user_id: &Uuid,
  content_md: &str,
) -> Result<CommentPub, LogicErr> {
  let visibility = match posts.fetch_visibility_by_id(post_id).await {
    Some(visibility) => visibility,
    None => return Err(LogicErr::MissingRecord),
  };

  let owner_id = match posts.fetch_owner_by_id(post_id).await {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  // If the commenting user doesn't own the post and the post isn't publicly available, don't let the user comment
  if (visibility == AccessType::Private || visibility == AccessType::Shadow) && &owner_id != user_id {
    return Err(LogicErr::UnauthorizedError);
  }

  // If the post is only available to the author's followers and the user isn't a follower of the author, don't let the
  // user comment
  if visibility == AccessType::FollowersOnly && !follows.user_follows_poster(post_id, user_id).await {
    return Err(LogicErr::MissingRecord);
  }

  let content_html = markdown::to_html(content_md);

  let comment_id = comments
    .create_comment(user_id, post_id, content_md, &content_html)
    .await?;

  match comments
    .fetch_comment(post_id, &comment_id, &Some(user_id.to_owned()))
    .await
  {
    Some(comment) => Ok(comment),
    None => Err(LogicErr::MissingRecord),
  }
}

pub async fn create_comment_like(
  posts: &PostPool,
  follows: &FollowPool,
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  let visibility = match posts.fetch_visibility_by_id(post_id).await {
    Some(visibility) => visibility,
    None => return Err(LogicErr::MissingRecord),
  };

  let owner_id = match posts.fetch_owner_by_id(post_id).await {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  // If the commenting user doesn't own the post and the post isn't publicly available, don't let the user comment
  if (visibility == AccessType::Private || visibility == AccessType::Shadow) && &owner_id != user_id {
    return Err(LogicErr::UnauthorizedError);
  }

  // If the post is only available to the author's followers and the user isn't a follower of the author, don't let the
  // user comment
  if visibility == AccessType::FollowersOnly && !follows.user_follows_poster(post_id, user_id).await {
    return Err(LogicErr::MissingRecord);
  }

  comments.create_comment_like(user_id, comment_id, post_id).await
}

pub async fn delete_comment(
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  comments.delete_comment(user_id, post_id, comment_id).await
}

pub async fn delete_comment_like(
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  comments.delete_comment_like(user_id, comment_id, post_id).await
}

pub async fn get_comments(
  comments: &CommentPool,
  post_id: &Uuid,
  own_user_id: &Option<Uuid>,
  page: &Option<i64>,
  page_size: &Option<i64>,
) -> Result<ListResponse<CommentPub>, LogicErr> {
  let page = page.unwrap_or(0);
  let page_size = page_size.unwrap_or(20);
  let comments_count = match comments.fetch_comments_count(post_id, own_user_id).await {
    Ok(count) => count,
    Err(err) => return Err(err),
  };

  if comments_count == 0 {
    return Err(LogicErr::MissingRecord);
  }

  match comments
    .fetch_comments(post_id, own_user_id, page_size, page * page_size)
    .await
  {
    Ok(comments) => Ok(ListResponse {
      data: comments,
      page,
      total_items: comments_count,
      total_pages: div_up(comments_count, page_size) + 1,
    }),
    Err(err) => Err(err),
  }
}

pub async fn activitypub_get_comments(
  posts: &PostPool,
  comments: &CommentPool,
  post_id: &Uuid,
  own_user_id: &Option<Uuid>,
  page: &Option<i64>,
  page_size: &Option<i64>,
) -> Result<ActivityPubDocument, LogicErr> {
  let author_handle = match posts.fetch_owner_handle_by_id(post_id).await {
    Some(h) => h,
    None => return Err(LogicErr::MissingRecord),
  };

  let page = page.unwrap_or(0);
  let page_size = page_size.unwrap_or(20);
  let comments_count = match comments.fetch_comments_count(post_id, own_user_id).await {
    Ok(count) => count,
    Err(err) => return Err(err),
  };

  if comments_count == 0 {
    return Err(LogicErr::MissingRecord);
  }

  match comments
    .fetch_comments(post_id, own_user_id, page_size, page * page_size)
    .await
  {
    Ok(comments) => Ok(create_activitypub_ordered_collection_page(
      &format!("{}/feed/{}/comments", SETTINGS.server.api_fqdn, post_id),
      page.try_into().unwrap_or_default(),
      page_size.try_into().unwrap_or_default(),
      comments_count.try_into().unwrap_or_default(),
      comments,
      Some(format!("{}/users/{}", SETTINGS.server.api_fqdn, author_handle)),
    )),
    Err(err) => Err(err),
  }
}

pub async fn get_comment(
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  own_user_id: &Option<Uuid>,
) -> Result<CommentPub, LogicErr> {
  match comments.fetch_comment(post_id, comment_id, own_user_id).await {
    Some(comment) => Ok(comment),
    None => Err(LogicErr::MissingRecord),
  }
}

pub async fn activitypub_get_comment(
  comments: &CommentPool,
  posts: &PostPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  own_user_id: &Option<Uuid>,
) -> Result<ActivityPubDocument, LogicErr> {
  let author_handle = match posts.fetch_owner_handle_by_id(post_id).await {
    Some(h) => h,
    None => return Err(LogicErr::MissingRecord),
  };

  let actor = format!("{}/users/{}", SETTINGS.server.api_fqdn, author_handle);

  match comments.fetch_comment(post_id, comment_id, own_user_id).await {
    Some(comment) => match comment.to_object(&actor) {
      Some(comment) => {
        let comment = ActivityPubDocument::new(comment);
        Ok(comment)
      }
      None => Err(LogicErr::MissingRecord),
    },
    None => Err(LogicErr::MissingRecord),
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use chrono::Utc;
  use mockall::predicate::*;
  use uuid::Uuid;

  use crate::{
    db::{
      comment_repository::{CommentPool, MockCommentRepo},
      follow_repository::{FollowPool, MockFollowRepo},
      post_repository::{MockPostRepo, PostPool},
    },
    logic::{
      comment::{create_comment, create_comment_like, delete_comment, delete_comment_like},
      LogicErr,
    },
    model::{access_type::AccessType, comment_pub::CommentPub},
  };

  #[async_std::test]
  async fn test_create_comment_rejects_for_missing_post() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| None);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_rejects_for_missing_post_owner() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

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
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_rejects_for_foreign_user_private_visibility() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_create_comment_rejects_for_foreign_user_shadow_visibility() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::Shadow));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(Uuid::new_v4()));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_create_comment_rejects_for_foreign_user_not_following_followers_only() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_user_follows_poster()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| false);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_rejects_for_db_err() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_create_comment()
      .times(1)
      .with(eq(user_id), eq(post_id), eq("test"), always())
      .returning(|_, _, _, _| Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      create_comment(&posts, &follows, &comments, &post_id, &user_id, "test").await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_comment_succeeds() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();
    let exp_comment_id = comment_id;
    let comment = Some(CommentPub {
      comment_id: exp_comment_id,
      user_id,
      post_id,
      content_md: "test".to_string(),
      content_html: "<p>test</p>".to_string(),
      created_at: Utc::now(),
      updated_at: Utc::now(),
      user_handle: "a".to_string(),
      user_fediverse_id: "a".to_string(),
      user_avatar_url: Some("a".to_string()),
      likes: 0,
      liked: Some(true),
      visibility: AccessType::PublicFederated,
    });

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_create_comment()
      .times(1)
      .with(eq(user_id), eq(post_id), eq("test"), always())
      .returning(move |_, _, _, _| Ok(comment_id));

    comment_repo
      .expect_fetch_comment()
      .times(1)
      .with(eq(post_id), eq(exp_comment_id), eq(Some(user_id)))
      .return_const(comment);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(comment_repo);

    assert!(create_comment(&posts, &follows, &comments, &post_id, &user_id, "test")
      .await
      .is_ok());
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_missing_post() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| None);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_missing_post_owner() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

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
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_foreign_user_private_visibility() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_foreign_user_shadow_visibility() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut post_repo = MockPostRepo::new();

    post_repo
      .expect_fetch_visibility_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(AccessType::Shadow));

    post_repo
      .expect_fetch_owner_by_id()
      .times(1)
      .with(eq(post_id))
      .returning(|_| Some(Uuid::new_v4()));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_foreign_user_not_following_followers_only() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut follow_repo = MockFollowRepo::new();

    follow_repo
      .expect_user_follows_poster()
      .times(1)
      .with(eq(post_id), eq(user_id))
      .returning(|_, _| false);

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(follow_repo);
    let comments: CommentPool = Arc::new(MockCommentRepo::new());

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_rejects_for_db_err() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_create_comment_like()
      .times(1)
      .with(eq(user_id), eq(comment_id), eq(post_id))
      .returning(|_, _, _| Err(LogicErr::DbError("Boop".to_string())));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_create_comment_like_succeeds() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

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
      .returning(|_| Some(Uuid::new_v4()));

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_create_comment_like()
      .times(1)
      .with(eq(user_id), eq(comment_id), eq(post_id))
      .returning(|_, _, _| Ok(()));

    let posts: PostPool = Arc::new(post_repo);
    let follows: FollowPool = Arc::new(MockFollowRepo::new());
    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      create_comment_like(&posts, &follows, &comments, &post_id, &comment_id, &user_id).await,
      Ok(())
    );
  }

  #[async_std::test]
  async fn test_delete_comment_err_passthrough() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_delete_comment()
      .times(1)
      .with(eq(user_id), eq(post_id), eq(comment_id))
      .returning(|_, _, _| Err(LogicErr::DbError("Boop".to_string())));

    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      delete_comment(&comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_delete_comment_succeeds() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_delete_comment()
      .times(1)
      .with(eq(user_id), eq(post_id), eq(comment_id))
      .returning(|_, _, _| Ok(()));

    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(delete_comment(&comments, &post_id, &comment_id, &user_id).await, Ok(()));
  }

  #[async_std::test]
  async fn test_delete_comment_like_err_passthrough() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_delete_comment_like()
      .times(1)
      .with(eq(user_id), eq(comment_id), eq(post_id))
      .returning(|_, _, _| Err(LogicErr::DbError("Boop".to_string())));

    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      delete_comment_like(&comments, &post_id, &comment_id, &user_id).await,
      Err(LogicErr::DbError("Boop".to_string()))
    );
  }

  #[async_std::test]
  async fn test_delete_comment_like_succeeds() {
    let post_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let comment_id = Uuid::new_v4();

    let mut comment_repo = MockCommentRepo::new();

    comment_repo
      .expect_delete_comment_like()
      .times(1)
      .with(eq(user_id), eq(comment_id), eq(post_id))
      .returning(|_, _, _| Ok(()));

    let comments: CommentPool = Arc::new(comment_repo);

    assert_eq!(
      delete_comment_like(&comments, &post_id, &comment_id, &user_id).await,
      Ok(())
    );
  }
}

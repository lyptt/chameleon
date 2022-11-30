use uuid::Uuid;

use crate::{
  db::{comment_repository::CommentPool, follow_repository::FollowPool, post_repository::PostPool},
  helpers::api::map_db_err,
  model::access_type::AccessType,
};

use super::LogicErr;

pub async fn create_comment(
  posts: &PostPool,
  follows: &FollowPool,
  comments: &CommentPool,
  post_id: &Uuid,
  user_id: &Uuid,
  content_md: &str,
) -> Result<Uuid, LogicErr> {
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

  comments
    .create_comment(user_id, post_id, content_md, &content_html)
    .await
    .map_err(map_db_err)
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

  comments
    .create_comment_like(user_id, comment_id, post_id)
    .await
    .map_err(map_db_err)
}

pub async fn delete_comment(
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  comments
    .delete_comment(user_id, post_id, comment_id)
    .await
    .map_err(map_db_err)
}

pub async fn delete_comment_like(
  comments: &CommentPool,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  comments
    .delete_comment_like(user_id, comment_id, post_id)
    .await
    .map_err(map_db_err)
}

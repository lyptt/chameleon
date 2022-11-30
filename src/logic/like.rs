use uuid::Uuid;

use crate::{
  db::{follow_repository::FollowPool, like_repository::LikePool, post_repository::PostPool},
  helpers::api::map_db_err,
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

  likes.create_like(user_id, post_id).await.map_err(map_db_err)
}

pub async fn delete_like(likes: &LikePool, post_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
  likes.delete_like(user_id, post_id).await.map_err(map_db_err)
}

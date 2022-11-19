use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  helpers::api::map_db_err,
  model::{access_type::AccessType, comment::Comment, follow::Follow, post::Post},
};

use super::LogicErr;

pub async fn create_comment(
  db: &Pool<Postgres>,
  post_id: &Uuid,
  user_id: &Uuid,
  content_md: &String,
) -> Result<Uuid, LogicErr> {
  let visibility = match Post::fetch_visibility_by_id(post_id, db).await {
    Some(visibility) => visibility,
    None => return Err(LogicErr::MissingRecord),
  };

  let owner_id = match Post::fetch_owner_by_id(post_id, db).await {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  // If the commenting user doesn't own the post and the post isn't publicly available, don't let the user comment
  if (visibility == AccessType::Private || visibility == AccessType::Shadow) && &owner_id != user_id {
    return Err(LogicErr::UnauthorizedError);
  }

  // If the post is only available to the author's followers and the user isn't a follower of the author, don't let the
  // user comment
  if visibility == AccessType::FollowersOnly && !Follow::user_follows_poster(post_id, user_id, db).await {
    return Err(LogicErr::MissingRecord);
  }

  let content_html = markdown::to_html(content_md);

  Comment::create_comment(user_id, post_id, content_md, &content_html, db)
    .await
    .map_err(map_db_err)
}

pub async fn delete_comment(
  db: &Pool<Postgres>,
  post_id: &Uuid,
  comment_id: &Uuid,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  Comment::delete_comment(user_id, post_id, comment_id, db)
    .await
    .map_err(map_db_err)
}

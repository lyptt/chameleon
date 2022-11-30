use uuid::Uuid;

use crate::{
  db::{follow_repository::FollowPool, user_repository::UserPool},
  helpers::api::map_db_err,
};

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

  follows
    .create_follow(user_id, &following_user_id)
    .await
    .map_err(map_db_err)
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

  follows
    .delete_follow(user_id, &following_user_id)
    .await
    .map_err(map_db_err)
}

use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  helpers::api::map_db_err,
  model::{follow::Follow, user::User},
};

use super::LogicErr;

pub async fn create_follow(
  db: &Pool<Postgres>,
  following_user_handle: &String,
  user_id: &Uuid,
) -> Result<Uuid, LogicErr> {
  let following_user_id = match User::fetch_id_by_handle(following_user_handle, &db).await {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  Follow::create_follow(user_id, &following_user_id, db)
    .await
    .map_err(map_db_err)
}

pub async fn delete_follow(
  db: &Pool<Postgres>,
  following_user_handle: &String,
  user_id: &Uuid,
) -> Result<(), LogicErr> {
  let following_user_id = match User::fetch_id_by_handle(following_user_handle, &db).await {
    Some(user_id) => user_id,
    None => return Err(LogicErr::MissingRecord),
  };

  Follow::delete_follow(user_id, &following_user_id, db)
    .await
    .map_err(map_db_err)
}

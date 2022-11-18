use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{helpers::api::map_db_err, model::like::Like};

use super::LogicErr;

pub async fn create_like(db: &Pool<Postgres>, post_id: &Uuid, user_id: &Uuid) -> Result<Uuid, LogicErr> {
  Like::create_like(user_id, post_id, db).await.map_err(map_db_err)
}

pub async fn delete_like(db: &Pool<Postgres>, post_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
  Like::delete_like(user_id, post_id, db).await.map_err(map_db_err)
}

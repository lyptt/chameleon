use actix_web::web;
use sqlx::PgPool;

use crate::model::user::User;

use super::LogicErr;

pub async fn get_user_by_id(handle: &String, db: &web::Data<PgPool>) -> Result<Option<User>, LogicErr> {
  User::fetch_by_handle(handle, db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_user_by_fediverse_id(fediverse_id: &String, db: &web::Data<PgPool>) -> Result<Option<User>, LogicErr> {
  User::fetch_by_fediverse_id(fediverse_id, db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

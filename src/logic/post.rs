use sqlx::{Pool, Postgres};

use super::LogicErr;
use crate::model::post_pub::PostPub;

pub async fn get_user_posts(
  handle: &str,
  limit: i64,
  skip: i64,
  db: &Pool<Postgres>,
) -> Result<Vec<PostPub>, LogicErr> {
  PostPub::fetch_user_own_feed(handle, limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_user_posts_count(handle: &str, db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostPub::count_user_own_feed(handle, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_global_posts(limit: i64, skip: i64, db: &Pool<Postgres>) -> Result<Vec<PostPub>, LogicErr> {
  PostPub::fetch_global_federated_feed(limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_global_posts_count(db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  PostPub::count_global_federated_feed(&db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

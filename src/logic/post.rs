use sqlx::{Pool, Postgres};

use super::LogicErr;
use crate::model::{access_type::AccessType, post::Post};

pub async fn get_user_posts(
  handle: &str,
  visibilities: Vec<AccessType>,
  limit: i64,
  skip: i64,
  db: &Pool<Postgres>,
) -> Result<Vec<Post>, LogicErr> {
  Post::fetch_by_user(handle, &visibilities, limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_user_posts_count(
  handle: &str,
  visibilities: Vec<AccessType>,
  db: &Pool<Postgres>,
) -> Result<i64, LogicErr> {
  Post::count_by_user(handle, &visibilities, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

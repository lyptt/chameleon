use sqlx::{Pool, Postgres};

use super::LogicErr;
use crate::model::post::Post;

pub async fn get_user_posts(handle: &str, limit: i64, skip: i64, db: &Pool<Postgres>) -> Result<Vec<Post>, LogicErr> {
  Post::fetch_user_own_feed(handle, limit, skip, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

pub async fn get_user_posts_count(handle: &str, db: &Pool<Postgres>) -> Result<i64, LogicErr> {
  Post::count_user_own_feed(handle, &db)
    .await
    .map_err(|e| LogicErr::DbError(e))
}

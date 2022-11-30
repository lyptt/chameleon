use std::sync::Arc;

use sqlx::{Pool, Postgres};

use super::{
  app_repository::{AppPool, DbAppRepo},
  comment_repository::{CommentPool, DbCommentRepo},
  event_repository::{DbEventRepo, EventPool},
  follow_repository::{DbFollowRepo, FollowPool},
  job_repository::{DbJobRepo, JobPool},
  like_repository::{DbLikeRepo, LikePool},
  post_repository::{DbPostRepo, PostPool},
  session_repository::{DbSessionRepo, SessionPool},
  user_repository::{DbUserRepo, UserPool},
  user_stats_repository::{DbUserStatsRepo, UserStatsPool},
};

pub struct Repository {}

impl Repository {
  pub fn new_app_pool(db: &Pool<Postgres>) -> AppPool {
    Arc::new(DbAppRepo { db: db.clone() })
  }

  pub fn new_comment_pool(db: &Pool<Postgres>) -> CommentPool {
    Arc::new(DbCommentRepo { db: db.clone() })
  }

  pub fn new_event_pool(db: &Pool<Postgres>) -> EventPool {
    Arc::new(DbEventRepo { db: db.clone() })
  }

  pub fn new_follow_pool(db: &Pool<Postgres>) -> FollowPool {
    Arc::new(DbFollowRepo { db: db.clone() })
  }

  pub fn new_job_pool(db: &Pool<Postgres>) -> JobPool {
    Arc::new(DbJobRepo { db: db.clone() })
  }

  pub fn new_like_pool(db: &Pool<Postgres>) -> LikePool {
    Arc::new(DbLikeRepo { db: db.clone() })
  }

  pub fn new_post_pool(db: &Pool<Postgres>) -> PostPool {
    Arc::new(DbPostRepo { db: db.clone() })
  }

  pub fn new_session_pool(db: &Pool<Postgres>) -> SessionPool {
    Arc::new(DbSessionRepo { db: db.clone() })
  }

  pub fn new_user_pool(db: &Pool<Postgres>) -> UserPool {
    Arc::new(DbUserRepo { db: db.clone() })
  }

  pub fn new_user_stats_pool(db: &Pool<Postgres>) -> UserStatsPool {
    Arc::new(DbUserStatsRepo { db: db.clone() })
  }
}

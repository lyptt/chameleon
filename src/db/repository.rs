use std::sync::Arc;

use deadpool_postgres::Pool;

use super::{
  app_repository::{AppPool, DbAppRepo},
  comment_repository::{CommentPool, DbCommentRepo},
  event_repository::{DbEventRepo, EventPool},
  follow_repository::{DbFollowRepo, FollowPool},
  job_repository::{DbJobRepo, JobPool},
  like_repository::{DbLikeRepo, LikePool},
  orbit_moderator_repository::{DbOrbitModeratorRepo, OrbitModeratorPool},
  orbit_repository::{DbOrbitRepo, OrbitPool},
  post_attachment_repository::{DbPostAttachmentRepo, PostAttachmentPool},
  post_repository::{DbPostRepo, PostPool},
  session_repository::{DbSessionRepo, SessionPool},
  user_orbit_repository::{DbUserOrbitRepo, UserOrbitPool},
  user_repository::{DbUserRepo, UserPool},
  user_stats_repository::{DbUserStatsRepo, UserStatsPool},
};

pub struct Repository {}

impl Repository {
  pub fn new_app_pool(db: &Pool) -> AppPool {
    Arc::new(DbAppRepo { db: db.clone() })
  }

  pub fn new_comment_pool(db: &Pool) -> CommentPool {
    Arc::new(DbCommentRepo { db: db.clone() })
  }

  pub fn new_event_pool(db: &Pool) -> EventPool {
    Arc::new(DbEventRepo { db: db.clone() })
  }

  pub fn new_follow_pool(db: &Pool) -> FollowPool {
    Arc::new(DbFollowRepo { db: db.clone() })
  }

  pub fn new_job_pool(db: &Pool) -> JobPool {
    Arc::new(DbJobRepo { db: db.clone() })
  }

  pub fn new_like_pool(db: &Pool) -> LikePool {
    Arc::new(DbLikeRepo { db: db.clone() })
  }

  pub fn new_post_pool(db: &Pool) -> PostPool {
    Arc::new(DbPostRepo { db: db.clone() })
  }

  pub fn new_post_attachment_pool(db: &Pool) -> PostAttachmentPool {
    Arc::new(DbPostAttachmentRepo { db: db.clone() })
  }

  pub fn new_session_pool(db: &Pool) -> SessionPool {
    Arc::new(DbSessionRepo { db: db.clone() })
  }

  pub fn new_user_pool(db: &Pool) -> UserPool {
    Arc::new(DbUserRepo { db: db.clone() })
  }

  pub fn new_user_stats_pool(db: &Pool) -> UserStatsPool {
    Arc::new(DbUserStatsRepo { db: db.clone() })
  }

  pub fn new_orbit_pool(db: &Pool) -> OrbitPool {
    Arc::new(DbOrbitRepo { db: db.clone() })
  }

  pub fn new_orbit_moderator_pool(db: &Pool) -> OrbitModeratorPool {
    Arc::new(DbOrbitModeratorRepo { db: db.clone() })
  }

  pub fn new_user_orbit_pool(db: &Pool) -> UserOrbitPool {
    Arc::new(DbUserOrbitRepo { db: db.clone() })
  }
}

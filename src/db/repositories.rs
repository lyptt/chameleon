use sqlx::{Pool, Postgres};

use super::{
  app_repository::AppPool, comment_repository::CommentPool, event_repository::EventPool, follow_repository::FollowPool,
  job_repository::JobPool, like_repository::LikePool, post_repository::PostPool, repository::Repository,
  session_repository::SessionPool, user_repository::UserPool, user_stats_repository::UserStatsPool,
};

#[derive(Clone)]
pub struct Repositories {
  pub apps: AppPool,
  pub comments: CommentPool,
  pub events: EventPool,
  pub follows: FollowPool,
  pub jobs: JobPool,
  pub likes: LikePool,
  pub posts: PostPool,
  pub sessions: SessionPool,
  pub users: UserPool,
  pub user_stats: UserStatsPool,
}

impl Repositories {
  pub fn new(db: &Pool<Postgres>) -> Self {
    Repositories {
      apps: Repository::new_app_pool(db),
      comments: Repository::new_comment_pool(db),
      events: Repository::new_event_pool(db),
      follows: Repository::new_follow_pool(db),
      jobs: Repository::new_job_pool(db),
      likes: Repository::new_like_pool(db),
      posts: Repository::new_post_pool(db),
      sessions: Repository::new_session_pool(db),
      users: Repository::new_user_pool(db),
      user_stats: Repository::new_user_stats_pool(db),
    }
  }
}

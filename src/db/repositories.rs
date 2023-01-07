use deadpool_postgres::Pool;

use super::{
  app_repository::AppPool, comment_repository::CommentPool, event_repository::EventPool, follow_repository::FollowPool,
  job_repository::JobPool, like_repository::LikePool, orbit_moderator_repository::OrbitModeratorPool,
  orbit_repository::OrbitPool, post_attachment_repository::PostAttachmentPool, post_repository::PostPool,
  repository::Repository, session_repository::SessionPool, tombstone_repository::TombstonePool,
  user_orbit_repository::UserOrbitPool, user_repository::UserPool, user_stats_repository::UserStatsPool,
};

#[derive(Clone)]
pub struct Repositories {
  pool: Pool,
  pub apps: AppPool,
  pub comments: CommentPool,
  pub events: EventPool,
  pub follows: FollowPool,
  pub jobs: JobPool,
  pub likes: LikePool,
  pub posts: PostPool,
  pub post_attachments: PostAttachmentPool,
  pub sessions: SessionPool,
  pub users: UserPool,
  pub user_stats: UserStatsPool,
  pub orbits: OrbitPool,
  pub orbit_moderators: OrbitModeratorPool,
  pub user_orbits: UserOrbitPool,
  pub tombstones: TombstonePool,
}

impl Repositories {
  pub fn new(db: Pool) -> Self {
    Repositories {
      apps: Repository::new_app_pool(&db),
      comments: Repository::new_comment_pool(&db),
      events: Repository::new_event_pool(&db),
      follows: Repository::new_follow_pool(&db),
      jobs: Repository::new_job_pool(&db),
      likes: Repository::new_like_pool(&db),
      posts: Repository::new_post_pool(&db),
      post_attachments: Repository::new_post_attachment_pool(&db),
      sessions: Repository::new_session_pool(&db),
      users: Repository::new_user_pool(&db),
      user_stats: Repository::new_user_stats_pool(&db),
      orbits: Repository::new_orbit_pool(&db),
      orbit_moderators: Repository::new_orbit_moderator_pool(&db),
      user_orbits: Repository::new_user_orbit_pool(&db),
      tombstones: Repository::new_tombstone_pool(&db),
      pool: db,
    }
  }
}

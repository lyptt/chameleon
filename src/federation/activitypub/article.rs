use crate::{
  activitypub::object::Object,
  db::{
    follow_repository::FollowPool, job_repository::JobPool, orbit_repository::OrbitPool,
    post_attachment_repository::PostAttachmentPool, post_repository::PostPool, user_orbit_repository::UserOrbitPool,
  },
  logic::LogicErr,
  model::{access_type::AccessType, user::User},
  work_queue::queue::Queue,
};

use super::util::FederateResult;

pub async fn federate_create_article(
  activity_object: Object,
  actor: &User,
  access: AccessType,
  follows: &FollowPool,
  posts: &PostPool,
  jobs: &JobPool,
  post_attachments: &PostAttachmentPool,
  orbits: &OrbitPool,
  user_orbits: &UserOrbitPool,
  queue: &Queue,
) -> Result<FederateResult, LogicErr> {
  todo!()
}

pub async fn federate_update_article(
  activity_object: Object,
  actor: &User,
  access: AccessType,
  posts: &PostPool,
  orbits: &OrbitPool,
  user_orbits: &UserOrbitPool,
) -> Result<FederateResult, LogicErr> {
  todo!()
}

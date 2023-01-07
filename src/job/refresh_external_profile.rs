use uuid::Uuid;

use crate::{
  activitypub::reference::Reference,
  db::{job_repository::JobPool, user_repository::UserPool},
  federation::activitypub::actor::federate_update_user_actor,
  logic::LogicErr,
};

pub async fn refresh_external_profile(users: &UserPool, jobs: &JobPool, job_id: Uuid) -> Result<(), LogicErr> {
  let job = match jobs.fetch_by_id(&job_id).await? {
    Some(job) => job,
    None => return Err(LogicErr::MissingRecord),
  };

  let user_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  let user = users.fetch_by_id(&user_id).await?;

  federate_update_user_actor(&Some(Reference::Remote(user.fediverse_uri)), users).await?;

  Ok(())
}

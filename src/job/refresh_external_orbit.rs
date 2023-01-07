use uuid::Uuid;

use crate::{
  activitypub::reference::Reference,
  db::{job_repository::JobPool, orbit_repository::OrbitPool},
  federation::activitypub::actor::federate_update_orbit_group,
  logic::LogicErr,
};

pub async fn refresh_external_orbit(orbits: &OrbitPool, jobs: &JobPool, job_id: Uuid) -> Result<(), LogicErr> {
  let job = match jobs.fetch_by_id(&job_id).await? {
    Some(job) => job,
    None => return Err(LogicErr::MissingRecord),
  };

  let orbit_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::MissingRecord),
  };

  let orbit = match orbits.fetch_orbit(&orbit_id).await? {
    Some(orbit) => orbit,
    None => return Err(LogicErr::MissingRecord),
  };

  federate_update_orbit_group(&Some(Reference::Remote(orbit.fediverse_uri)), orbits).await?;

  Ok(())
}

use crate::{
  db::{job_repository::JobPool, orbit_repository::OrbitPool},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn refresh_external_orbits(orbits: &OrbitPool, jobs: &JobPool, queue: &Queue) -> Result<(), LogicErr> {
  let refreshing_orbits = orbits.fetch_outdated_external_orbits().await?;
  for orbit in refreshing_orbits {
    let job_id = jobs
      .create(NewJob {
        created_by_id: None,
        status: JobStatus::NotStarted,
        record_id: Some(orbit),
        associated_record_id: None,
      })
      .await
      .map_err(map_db_err)?;

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::RefreshExternalOrbit)
      .build();

    queue.send_job(job).await?;
  }

  Ok(())
}

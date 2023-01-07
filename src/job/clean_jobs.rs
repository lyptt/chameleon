use crate::{db::job_repository::JobPool, logic::LogicErr};

pub async fn clean_jobs(jobs: &JobPool) -> Result<(), LogicErr> {
  jobs.purge_completed_jobs().await
}

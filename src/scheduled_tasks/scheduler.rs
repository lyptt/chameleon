use clokwerk::AsyncScheduler;
use std::time::Duration;
use tokio::task::JoinHandle;

use super::{
  task_trigger_clean_jobs_event::schedule_task_trigger_clean_jobs_event,
  task_trigger_refresh_external_orbits_event::schedule_task_trigger_refresh_external_orbits_event,
  task_trigger_refresh_external_profiles_event::schedule_task_trigger_refresh_external_profiles_event,
};

pub struct JobScheduler {
  handle: Option<JoinHandle<()>>,
}

impl JobScheduler {
  pub fn new() -> JobScheduler {
    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    schedule_task_trigger_clean_jobs_event(&mut scheduler);
    schedule_task_trigger_refresh_external_orbits_event(&mut scheduler);
    schedule_task_trigger_refresh_external_profiles_event(&mut scheduler);

    let handle = tokio::spawn(async move {
      loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
      }
    });

    JobScheduler { handle: Some(handle) }
  }
}

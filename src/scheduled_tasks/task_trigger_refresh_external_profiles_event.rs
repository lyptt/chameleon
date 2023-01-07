use chrono::Utc;
use clokwerk::{AsyncScheduler, TimeUnits};

use crate::helpers::api::map_db_err;
use crate::model::job::{JobStatus, NewJob};
use crate::model::queue_job::{QueueJob, QueueJobType};
use crate::worker_internal::services::{DB, QUEUE};

pub fn schedule_task_trigger_refresh_external_profiles_event(scheduler: &mut AsyncScheduler<Utc>) {
  scheduler.every(30.minutes()).run(move || async move {
    let job_id = match DB
      .jobs
      .create(NewJob {
        created_by_id: None,
        status: JobStatus::NotStarted,
        record_id: None,
        associated_record_id: None,
      })
      .await
      .map_err(map_db_err)
    {
      Ok(id) => id,
      Err(_) => return,
    };

    let job = QueueJob::builder()
      .job_id(job_id)
      .job_type(QueueJobType::RefreshExternalProfiles)
      .build();

    match QUEUE.send_job(job).await {
      Ok(_) => {}
      Err(err) => {
        log::error!("{}", err)
      }
    }
  });
}

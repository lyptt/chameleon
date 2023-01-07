use uuid::Uuid;

use crate::{
  db::{
    follow_repository::FollowPool, job_repository::JobPool, orbit_repository::OrbitPool, post_repository::PostPool,
    user_orbit_repository::UserOrbitPool, user_repository::UserPool,
  },
  federation::activitypub::{federate_ext, FederateExtAction, FederateExtActor, FederateExtActorRef},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    job::{JobStatus, NewJob},
    queue_job::{QueueJob, QueueJobType},
  },
  work_queue::queue::Queue,
};

pub async fn delete_post(
  job_id: Uuid,
  jobs: &JobPool,
  orbits: &OrbitPool,
  user_orbits: &UserOrbitPool,
  users: &UserPool,
  posts: &PostPool,
  follows: &FollowPool,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let job = match jobs.fetch_optional_by_id(&job_id).await {
    Some(job) => job,
    None => return Err(LogicErr::InternalError("Job not found".to_string())),
  };

  let user_id = match job.created_by_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("User not found".to_string())),
  };

  let post_id = match job.record_id {
    Some(id) => id,
    None => return Err(LogicErr::InternalError("Post ID not found for job".to_string())),
  };

  if let Some(orbit_id) = job.associated_record_id {
    match orbits.fetch_orbit(&orbit_id).await? {
      Some(orbit) => {
        let user = users.fetch_by_id(&user_id).await?;

        if orbit.is_external {
          federate_ext(
            FederateExtAction::DeletePost(post_id),
            &user,
            &FederateExtActor::Group(orbit),
            posts,
            orbits,
          )
          .await?;
          return Ok(());
        }
      }
      _ => {
        log::warn!(
          "Failed to fetch remote orbit information with id {} to federate post {}. Federation will be permanently aborted for this post.",
          orbit_id,
          post_id
        );
      }
    };

    let users = user_orbits.fetch_orbit_external_user_ids(&orbit_id).await?;

    for user in users {
      let job_id = jobs
        .create(NewJob {
          created_by_id: Some(user_id),
          status: JobStatus::NotStarted,
          record_id: Some(post_id),
          associated_record_id: Some(user),
        })
        .await
        .map_err(map_db_err)?;

      let job = QueueJob::builder()
        .job_id(job_id)
        .job_type(QueueJobType::FederateActivityPubExt)
        .context(vec![user_id.to_string()])
        .activitypub_federate_ext_action(FederateExtAction::DeletePost(post_id))
        .activitypub_federate_ext_dest_actor(FederateExtActorRef::Person(user))
        .build();

      queue.send_job(job).await?;
    }
  } else {
    let followers = follows.fetch_user_followers(&user_id).await.unwrap_or_default();

    for follower in followers {
      let job_id = jobs
        .create(NewJob {
          created_by_id: Some(user_id),
          status: JobStatus::NotStarted,
          record_id: Some(post_id),
          associated_record_id: Some(follower.user_id),
        })
        .await
        .map_err(map_db_err)?;

      let job = QueueJob::builder()
        .job_id(job_id)
        .job_type(QueueJobType::FederateActivityPubExt)
        .context(vec![user_id.to_string()])
        .activitypub_federate_ext_action(FederateExtAction::DeletePost(post_id))
        .activitypub_federate_ext_dest_actor(FederateExtActorRef::Person(follower.user_id))
        .build();

      queue.send_job(job).await?;
    }
  }

  Ok(())
}

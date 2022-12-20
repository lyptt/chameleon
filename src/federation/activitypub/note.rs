use uuid::Uuid;

use super::util::{activitypub_ref_to_uri_opt, deref_activitypub_ref};
use crate::{
  activitypub::{
    object::{Object, ObjectType},
    rdf_string::RdfString,
  },
  db::{follow_repository::FollowPool, job_repository::JobPool, post_repository::PostPool},
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    post::Post,
    queue_job::{QueueJob, QueueJobType},
    user::User,
  },
  work_queue::queue::Queue,
};

pub async fn federate_create_note(
  activity_object: Object,
  actor: User,
  access: AccessType,
  follows: &FollowPool,
  posts: &PostPool,
  jobs: &JobPool,
  queue: &Queue,
) -> Result<(), LogicErr> {
  let followers = follows.fetch_user_followers(&actor.user_id).await.unwrap_or_default();
  // Skip federating posts from users that our instance's users don't follow
  if followers.is_empty() {
    return Ok(());
  }

  let uri = match activitypub_ref_to_uri_opt(&activity_object.url) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  // TODO: Figure out some presentation method for text posts
  let attachment_obj = match deref_activitypub_ref(&activity_object.attachment).await {
    Some(obj) => {
      let obj_type = match ObjectType::from_str_opt(&obj.kind) {
        Some(t) => t,
        None => return Err(LogicErr::InvalidData),
      };

      if obj_type != ObjectType::Image {
        return Err(LogicErr::Unimplemented);
      }

      obj
    }
    None => return Err(LogicErr::Unimplemented),
  };

  let content_html: Option<RdfString> = match activity_object.content_map {
    Some(content) => {
      // TODO: Once we support multiple content languages for a post we should
      //       do a mapping here instead of pulling out english values
      if content.contains_key("en") {
        content.get("en").cloned()
      } else if content.contains_key("en-US") {
        content.get("en-US").cloned()
      } else if content.contains_key("en-GB") {
        content.get("en-GB").cloned()
      } else {
        None
      }
    }
    None => None,
  };

  let content_html = match content_html {
    Some(content) => match content {
      RdfString::Raw(content) => content,
      RdfString::Props(props) => props.string,
    },
    None => match activity_object.content {
      Some(content) => match content {
        RdfString::Raw(content) => content,
        RdfString::Props(props) => props.string,
      },
      None => "".to_string(),
    },
  };

  let content_md = match activity_object.source {
    Some(source) => {
      if source.media_type == "text/markdown" {
        source.content
      } else {
        "".to_string()
      }
    }
    None => "".to_string(),
  };

  let created_at = match activity_object.published {
    Some(date) => date,
    None => return Err(LogicErr::InvalidData),
  };

  let image_content_type = match attachment_obj.media_type {
    Some(val) => val,
    None => return Err(LogicErr::InvalidData),
  };

  let image_uri = match activitypub_ref_to_uri_opt(&attachment_obj.url) {
    Some(val) => val,
    None => return Err(LogicErr::InvalidData),
  };

  let post_id = Uuid::new_v4();

  let post = Post {
    post_id,
    user_id: actor.user_id,
    uri,
    is_external: true,
    content_md,
    content_html,
    content_image_uri_small: None,
    content_image_uri_medium: None,
    content_image_uri_large: Some(image_uri),
    content_width_small: None,
    content_width_medium: None,
    content_width_large: Some(4096),
    content_height_small: None,
    content_height_medium: None,
    content_height_large: Some(4096),
    content_type_small: None,
    content_type_medium: None,
    content_type_large: Some(image_content_type),
    content_image_storage_ref: None,
    content_blurhash: None,
    visibility: access,
    created_at,
    updated_at: created_at,
    deletion_scheduled_at: None,
  };

  posts.create_post_from(post).await?;

  let job_id = jobs
    .create(NewJob {
      created_by_id: Some(actor.user_id),
      status: JobStatus::NotStarted,
      record_id: Some(post_id),
      associated_record_id: None,
    })
    .await
    .map_err(map_db_err)?;

  let job = QueueJob::builder()
    .job_id(job_id)
    .job_type(QueueJobType::CreatePostEvents)
    .build();

  queue.send_job(job).await?;

  Ok(())
}

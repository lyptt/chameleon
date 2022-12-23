use uuid::Uuid;

use super::util::{activitypub_ref_to_uri_opt, deref_activitypub_ref, FederateResult};
use crate::{
  activitypub::{
    object::{Object, ObjectType},
    rdf_string::RdfString,
  },
  db::{follow_repository::FollowPool, job_repository::JobPool, like_repository::LikePool, post_repository::PostPool},
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
  actor: &User,
  access: AccessType,
  follows: &FollowPool,
  posts: &PostPool,
  jobs: &JobPool,
  queue: &Queue,
) -> Result<FederateResult, LogicErr> {
  let followers = follows.fetch_user_followers(&actor.user_id).await.unwrap_or_default();
  // Skip federating posts from users that our instance's users don't follow
  if followers.is_empty() {
    return Ok(FederateResult::None);
  }

  let uri = match activity_object.id {
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

      if obj_type != ObjectType::Image && obj_type != ObjectType::Document {
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

  Ok(FederateResult::None)
}

pub async fn federate_update_note(
  activity_object: Object,
  actor: &User,
  access: AccessType,
  posts: &PostPool,
) -> Result<FederateResult, LogicErr> {
  let uri = match activity_object.id {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let mut post = match posts.find_optional_by_uri(&uri).await {
    Some(post) => post,
    None => return Err(LogicErr::MissingRecord),
  };

  if !post.is_external || post.user_id != actor.user_id {
    return Err(LogicErr::UnauthorizedError);
  }

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

  post.content_image_uri_large = Some(image_uri);
  post.content_type_large = Some(image_content_type);
  post.content_html = content_html;
  post.content_md = content_md;
  post.visibility = access;
  post.created_at = created_at;
  post.updated_at = created_at;

  posts.update_post_content(&post).await?;

  Ok(FederateResult::None)
}

pub async fn federate_delete_note(target: String, actor: &User, posts: &PostPool) -> Result<FederateResult, LogicErr> {
  posts.delete_post_from_uri(&target, &actor.user_id).await?;

  Ok(FederateResult::None)
}

pub async fn federate_like_note(
  activity_object: Object,
  actor: &User,
  posts: &PostPool,
  likes: &LikePool,
) -> Result<FederateResult, LogicErr> {
  let uri = match activitypub_ref_to_uri_opt(&activity_object.url) {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let post = match posts.fetch_post_from_uri(&uri, &Some(actor.user_id)).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return Err(LogicErr::MissingRecord),
    },
    Err(err) => return Err(map_db_err(err)),
  };

  likes.create_like(&actor.user_id, &post.post_id).await.map(|_| ())?;

  Ok(FederateResult::None)
}

pub async fn federate_unlike_note(
  target: String,
  actor: &User,
  posts: &PostPool,
  likes: &LikePool,
) -> Result<FederateResult, LogicErr> {
  let post = match posts.fetch_post_from_uri(&target, &Some(actor.user_id)).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return Err(LogicErr::MissingRecord),
    },
    Err(err) => return Err(map_db_err(err)),
  };

  likes.delete_like(&actor.user_id, &post.post_id).await.map(|_| ())?;

  Ok(FederateResult::None)
}

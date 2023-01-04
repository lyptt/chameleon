use uuid::Uuid;

use crate::{
  activitypub::{
    activity::ActivityProps,
    activity_convertible::ActivityConvertible,
    activity_type::ActivityType,
    document::ActivityPubDocument,
    object::{Object, ObjectType},
    rdf_string::RdfString,
    reference::Reference,
  },
  db::{
    job_repository::JobPool, orbit_repository::OrbitPool, post_attachment_repository::PostAttachmentPool,
    post_repository::PostPool, user_orbit_repository::UserOrbitPool,
  },
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{
    access_type::AccessType,
    job::{JobStatus, NewJob},
    orbit::Orbit,
    post::Post,
    post_attachment::PostAttachment,
    queue_job::{QueueJob, QueueJobType},
    user::User,
  },
  settings::SETTINGS,
  work_queue::queue::Queue,
};

use super::{
  actor::federate_orbit_group,
  util::{activitypub_ref_to_uri_opt, deref_activitypub_ref_list, send_activitypub_object, FederateResult},
};

pub async fn federate_create_article(
  activity_object: Object,
  actor: &User,
  posts: &PostPool,
  jobs: &JobPool,
  post_attachments: &PostAttachmentPool,
  orbits: &OrbitPool,
  user_orbits: &UserOrbitPool,
  queue: &Queue,
) -> Result<FederateResult, LogicErr> {
  let orbit = federate_orbit_group(&activity_object.audience, orbits).await?;

  let members = user_orbits.fetch_orbit_user_ids(&orbit.orbit_id).await?;

  // Skip federating posts from users that our instance's users don't follow or orbits that have no followers
  if members.is_empty() {
    return Ok(FederateResult::None);
  }

  let uri = match activity_object.id {
    Some(uri) => uri,
    None => return Err(LogicErr::InvalidData),
  };

  let attachments: Vec<Object> = match deref_activitypub_ref_list(&activity_object.attachment).await {
    Some(obj) => obj
      .into_iter()
      .filter(|obj| {
        let obj_type = match ObjectType::from_str_opt(&obj.kind) {
          Some(t) => t,
          None => return false,
        };

        if obj_type != ObjectType::Image && obj_type != ObjectType::Document {
          return false;
        }

        true
      })
      .collect(),
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

  let title = match activity_object.summary {
    Some(title) => match title {
      RdfString::Raw(title) => Some(title),
      RdfString::Props(props) => Some(props.string),
    },
    None => None,
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

  let post_id = Uuid::new_v4();

  let post = Post {
    post_id,
    user_id: actor.user_id,
    orbit_id: Some(orbit.orbit_id),
    uri,
    is_external: true,
    title,
    content_md,
    content_html,
    // NOTE: It doesn't make sense to have unlisted posts in an orbit, so we hardcode this to PublicFederated here.
    visibility: AccessType::PublicFederated,
    created_at,
    updated_at: created_at,
    deletion_scheduled_at: None,
  };

  posts.create_post_from(post).await?;

  for attachment_obj in attachments {
    let image_content_type = match attachment_obj.media_type {
      Some(val) => val,
      None => continue,
    };

    let image_width: i32 = match attachment_obj.width {
      Some(val) => val.try_into().unwrap_or_default(),
      None => continue,
    };

    let image_height: i32 = match attachment_obj.height {
      Some(val) => val.try_into().unwrap_or_default(),
      None => continue,
    };

    let image_uri = match activitypub_ref_to_uri_opt(&attachment_obj.url) {
      Some(val) => val,
      None => continue,
    };

    let attachment = PostAttachment {
      attachment_id: Uuid::new_v4(),
      user_id: actor.user_id,
      post_id,
      uri: Some(image_uri),
      width: image_width,
      height: image_height,
      content_type: Some(image_content_type),
      storage_ref: None,
      blurhash: None,
      created_at,
    };

    match post_attachments.create_attachment_from(attachment).await {
      Ok(_) => {}
      Err(err) => log::error!("Failed to create attachment for post {}: {:?}", post_id, err),
    }
  }

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

pub async fn federate_update_article(
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

  post.content_html = content_html;
  post.content_md = content_md;
  post.visibility = access;
  post.created_at = created_at;
  post.updated_at = created_at;

  posts.update_post_content(&post).await?;

  Ok(FederateResult::None)
}

pub async fn federate_ext_create_article(
  post_id: &Uuid,
  actor: &User,
  dest_actor: &Orbit,
  posts: &PostPool,
) -> Result<(), LogicErr> {
  let post = match posts.fetch_post(post_id, &Some(actor.user_id)).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return Err(LogicErr::MissingRecord),
    },
    Err(err) => {
      println!("{}", err);
      return Err(err);
    }
  };

  let obj = match post.to_object(&actor.fediverse_uri) {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Create.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(obj))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &dest_actor.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

pub async fn federate_ext_update_article(
  post_id: &Uuid,
  actor: &User,
  dest_actor: &Orbit,
  posts: &PostPool,
) -> Result<(), LogicErr> {
  let post = match posts.fetch_post(post_id, &Some(actor.user_id)).await {
    Ok(post) => match post {
      Some(post) => post,
      None => return Err(LogicErr::MissingRecord),
    },
    Err(err) => {
      println!("{}", err);
      return Err(err);
    }
  };

  let obj = match post.to_object(&actor.fediverse_uri) {
    Some(obj) => obj,
    None => return Err(LogicErr::MissingRecord),
  };

  let response_object = Object::builder()
    .kind(Some(ActivityType::Update.to_string()))
    .id(Some(format!("{}/{}", SETTINGS.server.api_fqdn, Uuid::new_v4())))
    .actor(Some(Reference::Remote(format!(
      "{}{}",
      SETTINGS.server.api_fqdn, actor.fediverse_uri
    ))))
    .activity(Some(
      ActivityProps::builder()
        .object(Some(Reference::Embedded(Box::new(obj))))
        .build(),
    ))
    .build();

  let doc = ActivityPubDocument::new(response_object);

  let response_uri = match &dest_actor.ext_apub_inbox_uri {
    Some(uri) => uri,
    None => return Ok(()),
  };

  send_activitypub_object(response_uri, doc, &actor.fediverse_uri, &actor.private_key).await
}

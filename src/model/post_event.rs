use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
  collections::{hash_map::Entry::Vacant, HashMap},
  str::FromStr,
};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible,
    collection::CollectionProps,
    object::{Object, ObjectSource},
    rdf_string::RdfString,
    reference::Reference,
  },
  db::{FromRow, FromRowJoin, FromRows},
  logic::LogicErr,
  settings::SETTINGS,
};

use super::{access_type::AccessType, event_type::EventType, post_attachment::PostAttachment};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct PostEvent {
  // Event columns
  pub event_type: EventType,
  // Post columns
  pub post_id: Uuid,
  pub orbit_id: Option<Uuid>,
  pub uri: String,
  pub content_md: String,
  pub content_html: String,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  // Foreign columns
  pub user_id: Uuid,
  pub user_handle: String,
  pub user_fediverse_id: String,
  pub user_fediverse_uri: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_avatar_url: Option<String>,
  pub event_user_handle: String,
  pub event_user_fediverse_id: String,
  pub event_user_fediverse_uri: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub event_user_avatar_url: Option<String>,
  pub likes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liked: Option<bool>,
  pub comments: i64,
  pub orbit_name: Option<String>,
  pub orbit_uri: Option<String>,
  pub orbit_avatar_uri: Option<String>,
  pub attachments: Vec<PostAttachment>,
}

impl FromRow for PostEvent {
  fn from_row(row: Row) -> Option<Self> {
    Some(PostEvent {
      event_type: EventType::from_str(row.get("event_type")).unwrap_or_default(),
      post_id: row.get("post_id"),
      uri: row.get("uri"),
      content_md: row.get("content_md"),
      content_html: row.get("content_html"),
      visibility: AccessType::from_str(row.get("visibility")).unwrap_or_default(),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      user_id: row.get("user_id"),
      user_handle: row.get("user_handle"),
      user_fediverse_id: row.get("user_fediverse_id"),
      user_fediverse_uri: row.get("user_fediverse_uri"),
      user_avatar_url: row.get("user_avatar_url"),
      event_user_handle: row.get("event_user_handle"),
      event_user_fediverse_id: row.get("event_user_fediverse_id"),
      event_user_fediverse_uri: row.get("event_user_fediverse_uri"),
      event_user_avatar_url: row.get("event_user_avatar_url"),
      likes: row.get("likes"),
      liked: row.get("liked"),
      comments: row.get("comments"),
      orbit_id: row.get("orbit_id"),
      orbit_name: row.get("orbit_name"),
      orbit_uri: row.get("orbit_uri"),
      orbit_avatar_uri: row.get("orbit_avatar_uri"),
      attachments: vec![],
    })
  }
}

impl FromRows for PostEvent {
  fn from_rows(rows: Vec<Row>) -> Result<Vec<Self>, LogicErr> {
    let mut ret: Vec<PostEvent> = vec![];
    let mut lookup = HashMap::<Uuid, usize>::new();

    for row in rows.into_iter() {
      let post_id: Uuid = row.get("post_id");
      if let Vacant(e) = lookup.entry(post_id) {
        let attachment = match row.get::<&str, Option<Uuid>>("attachment_id") {
          Some(_) => PostAttachment::from_row_join(&row),
          None => None,
        };

        let mut post = match PostEvent::from_row(row) {
          Some(post) => post,
          None => continue,
        };

        if let Some(attachment) = attachment {
          post.attachments.push(attachment);
        }

        e.insert(ret.len());
        ret.push(post);
      } else {
        let post = match ret.get_mut(lookup[&post_id]) {
          Some(post) => post,
          None => continue,
        };

        if let Some(attachment) = PostAttachment::from_row_join(&row) {
          post.attachments.push(attachment);
        }
      }
    }

    Ok(ret)
  }
}

impl ActivityConvertible for PostEvent {
  fn to_object(&self, actor: &str) -> Option<Object> {
    let actor_uri = actor.to_string();
    let actor_feed_uri = format!("{}/feed", actor);
    let actor_follower_feed_uri = format!("{}/followers", actor);

    let to = match self.visibility {
      AccessType::Shadow => Some(Reference::Remote::<Object>(actor_feed_uri)),
      AccessType::Unlisted => None,
      AccessType::Private => Some(Reference::Remote::<Object>(actor_uri.clone())),
      AccessType::FollowersOnly => Some(Reference::Remote::<Object>(actor_follower_feed_uri)),
      AccessType::PublicLocal => Some(Reference::Remote::<Object>(
        "https://www.w3.org/ns/activitystreams#Local".to_string(),
      )),
      AccessType::PublicFederated => Some(Reference::Remote::<Object>(
        "https://www.w3.org/ns/activitystreams#Public".to_string(),
      )),
      _ => None,
    };

    let cc = match self.visibility {
      AccessType::Unlisted => Some(Reference::Remote::<Object>(
        "https://www.w3.org/ns/activitystreams#Local".to_string(),
      )),
      _ => None,
    };

    let base_uri = format!("{}/feed/{}", SETTINGS.server.api_fqdn, self.post_id);

    let replies_collection = Object::builder()
      .kind(Some("OrderedCollection".to_string()))
      .id(Some(format!("{}/comments", base_uri)))
      .collection(Some(
        CollectionProps::builder()
          .items(Some(Reference::Remote(format!(
            "{}/comments?page=0&page_size=20",
            base_uri
          ))))
          .build(),
      ))
      .build();

    let attachment_refs = self
      .attachments
      .iter()
      .flat_map(|a| {
        if let Some(uri) = &a.uri {
          if let Some(content_type) = &a.content_type {
            let abs_uri = match uri.starts_with("http") {
              true => uri.clone(),
              false => format!("{}/{}", SETTINGS.server.cdn_fqdn, uri),
            };

            Some(Reference::Embedded(Box::new(
              Object::builder()
                .kind(Some("Image".to_string()))
                .media_type(Some(content_type.clone()))
                .width(Some(a.width.try_into().unwrap_or_default()))
                .height(Some(a.height.try_into().unwrap_or_default()))
                .url(Some(Reference::Remote(abs_uri)))
                .build(),
            )))
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();

    Some(
      Object::builder()
        .id(Some(base_uri.clone()))
        .url(Some(Reference::Remote(base_uri)))
        .kind(Some("Note".to_string()))
        .attributed_to(Some(Reference::Remote(actor_uri)))
        .to(to)
        .cc(cc)
        .replies(Some(Box::new(replies_collection)))
        .content(Some(RdfString::Raw(self.content_html.clone())))
        .source(Some(
          ObjectSource::builder()
            .content(self.content_md.clone())
            .media_type("text/markdown".to_string())
            .build(),
        ))
        .published(Some(self.created_at))
        .attachment(Some(Reference::Mixed(attachment_refs)))
        .build(),
    )
  }
}

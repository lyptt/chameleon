use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible,
    collection::CollectionProps,
    object::{Object, ObjectSource},
    rdf_string::RdfString,
    reference::Reference,
  },
  settings::SETTINGS,
};

use super::{access_type::AccessType, event_type::EventType};

#[derive(Deserialize, Serialize, FromRow, Debug, PartialEq, Eq, Clone)]
pub struct PostEvent {
  // Event columns
  pub event_type: EventType,
  // Post columns
  pub post_id: Uuid,
  pub uri: String,
  pub content_md: String,
  pub content_html: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_image_uri_small: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_image_uri_medium: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_image_uri_large: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_width_small: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_width_medium: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_width_large: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_height_small: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_height_medium: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_height_large: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type_small: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type_medium: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type_large: Option<String>,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_blurhash: Option<String>,
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

    let mut attachment_refs = vec![];

    if let Some(uri) = &self.content_image_uri_large {
      if let Some(width) = self.content_width_large {
        if let Some(height) = self.content_height_large {
          if let Some(content_type) = &self.content_type_large {
            let abs_uri = match uri.starts_with("http") {
              true => uri.clone(),
              false => format!("{}/{}", SETTINGS.server.cdn_fqdn, uri),
            };

            attachment_refs.push(Reference::Embedded(Box::new(
              Object::builder()
                .kind(Some("Image".to_string()))
                .media_type(Some(content_type.clone()))
                .width(Some(width.try_into().unwrap_or_default()))
                .height(Some(height.try_into().unwrap_or_default()))
                .url(Some(Reference::Remote(abs_uri)))
                .build(),
            )))
          }
        }
      }
    }

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

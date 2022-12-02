use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::activitypub::{
  activity::Activity, activity_convertible::ActivityConvertible, activity_type::ActivityType, image::Image, link::Link,
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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_avatar_url: Option<String>,
  pub event_user_handle: String,
  pub event_user_fediverse_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub event_user_avatar_url: Option<String>,
  pub likes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liked: Option<bool>,
  pub comments: i64,
}

impl ActivityConvertible<Image> for PostEvent {
  fn to_activity(&self, base_uri: &str, actor_uri: &str) -> Option<Activity<Image>> {
    let mut image_links: Vec<Link> = vec![];
    if let Some(link) = Link::from_post_pub_small(self) {
      image_links.push(link);
    };
    if let Some(link) = Link::from_post_pub_medium(self) {
      image_links.push(link);
    };
    if let Some(link) = Link::from_post_pub_large(self) {
      image_links.push(link);
    };

    if image_links.is_empty() {
      return None;
    }

    Some(Activity {
      id: format!("{}/{}", &base_uri, &self.uri),
      actor: actor_uri.to_string(),
      published: self.created_at,
      object: Image {
        to: Some(vec!["https://www.w3.org/ns/activitystreams#Public".to_string()]),
        cc: Some(vec![format!("{}/followers", base_uri)]),
        url: image_links,
        name: None,
        content: Some(self.content_html.clone()),
        object_type: "Image",
      },
      activity_type: ActivityType::Create,
      to: Some(vec!["https://www.w3.org/ns/activitystreams#Public".to_string()]),
      cc: Some(vec![format!("{}/followers", base_uri)]),
    })
  }
}

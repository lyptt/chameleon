use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible,
    object::{Object, ObjectSource},
    rdf_string::RdfString,
    reference::Reference,
  },
  settings::SETTINGS,
};

use super::access_type::AccessType;

#[derive(Deserialize, Serialize, FromRow)]
/// Represents a user's comment on a post
pub struct CommentPub {
  pub comment_id: Uuid,
  pub user_id: Uuid,
  pub post_id: Uuid,
  pub content_md: String,
  pub content_html: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub user_handle: String,
  pub user_fediverse_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_avatar_url: Option<String>,
  pub likes: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liked: Option<bool>,
  #[serde(skip)]
  pub visibility: AccessType,
}

impl ActivityConvertible for CommentPub {
  fn to_object(&self, actor: &str) -> Option<Object> {
    let id = format!(
      "{}/feed/{}/comments/{}",
      SETTINGS.server.api_fqdn, self.post_id, self.comment_id
    );

    let attributed_to_uri = format!("{}/users/{}", SETTINGS.server.api_fqdn, self.user_handle);
    let cc_uri = format!("{}/followers", actor);
    let in_reply_to_uri = format!("{}/feed/{}", SETTINGS.server.api_fqdn, self.post_id);

    let to = match self.visibility {
      AccessType::Shadow => None,
      AccessType::Unlisted => None,
      AccessType::Private => None,
      AccessType::FollowersOnly => Some(Reference::Remote::<Object>(cc_uri.clone())),
      AccessType::PublicLocal => Some(Reference::Remote::<Object>(
        "https://www.w3.org/ns/activitystreams#Local".to_string(),
      )),
      AccessType::PublicFederated => Some(Reference::Remote::<Object>(
        "https://www.w3.org/ns/activitystreams#Public".to_string(),
      )),
      _ => None,
    };

    Some(
      Object::builder()
        .id(Some(id.clone()))
        .kind(Some("Note".to_string()))
        .url(Some(Reference::Remote(id)))
        .attributed_to(Some(Reference::Remote(attributed_to_uri)))
        .cc(Some(Reference::Mixed(vec![Reference::Remote(cc_uri)])))
        .to(to)
        .content(Some(RdfString::Raw(self.content_html.clone())))
        .source(Some(
          ObjectSource::builder()
            .content(self.content_md.clone())
            .media_type("text/markdown".to_string())
            .build(),
        ))
        .in_reply_to(Some(Reference::Remote(in_reply_to_uri)))
        .published(Some(self.created_at))
        .build(),
    )
  }
}

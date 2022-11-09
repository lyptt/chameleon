use serde::{Deserialize, Serialize};

use crate::model::post::Post;

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
  #[serde(rename = "type")]
  pub object_type: String,
  pub href: String,
  #[serde(rename = "mediaType")]
  pub media_type: Option<String>,
  pub width: Option<i32>,
  pub height: Option<i32>,
}

impl Link {
  pub fn from_post_small(post: &Post) -> Option<Link> {
    match &post.content_image_url_small {
      Some(uri) => Some(Link {
        object_type: "Link".to_string(),
        href: uri.to_string(),
        media_type: post.content_type_small.clone(),
        width: post.content_width_small,
        height: post.content_height_small,
      }),
      None => None,
    }
  }

  pub fn from_post_medium(post: &Post) -> Option<Link> {
    match &post.content_image_url_medium {
      Some(uri) => Some(Link {
        object_type: "Link".to_string(),
        href: uri.to_string(),
        media_type: post.content_type_medium.clone(),
        width: post.content_width_medium,
        height: post.content_height_medium,
      }),
      None => None,
    }
  }

  pub fn from_post_large(post: &Post) -> Option<Link> {
    match &post.content_image_url_large {
      Some(uri) => Some(Link {
        object_type: "Link".to_string(),
        href: uri.to_string(),
        media_type: post.content_type_large.clone(),
        width: post.content_width_large,
        height: post.content_height_large,
      }),
      None => None,
    }
  }
}

use serde::{Deserialize, Serialize};

use crate::model::post_pub::PostPub;

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
  pub fn from_post_pub_small(post: &PostPub) -> Option<Link> {
    post.content_image_uri_small.as_ref().map(|uri| Link {
      object_type: "Link".to_string(),
      href: uri.to_string(),
      media_type: post.content_type_small.clone(),
      width: post.content_width_small,
      height: post.content_height_small,
    })
  }

  pub fn from_post_pub_medium(post: &PostPub) -> Option<Link> {
    post.content_image_uri_medium.as_ref().map(|uri| Link {
      object_type: "Link".to_string(),
      href: uri.to_string(),
      media_type: post.content_type_medium.clone(),
      width: post.content_width_medium,
      height: post.content_height_medium,
    })
  }

  pub fn from_post_pub_large(post: &PostPub) -> Option<Link> {
    post.content_image_uri_large.as_ref().map(|uri| Link {
      object_type: "Link".to_string(),
      href: uri.to_string(),
      media_type: post.content_type_large.clone(),
      width: post.content_width_large,
      height: post.content_height_large,
    })
  }
}

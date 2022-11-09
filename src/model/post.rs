use crate::activitypub::{
  activity::Activity, activity_convertible::ActivityConvertible, activity_type::ActivityType, image::Image, link::Link,
};

use super::access_type::AccessType;

use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Post {
  pub post_id: Uuid,
  pub user_id: Uuid,
  pub uri: String,
  pub is_external: bool,
  pub content_md: String,
  pub content_html: String,
  pub content_image_url_small: Option<String>,
  pub content_image_url_medium: Option<String>,
  pub content_image_url_large: Option<String>,
  pub content_width_small: Option<i32>,
  pub content_width_medium: Option<i32>,
  pub content_width_large: Option<i32>,
  pub content_height_small: Option<i32>,
  pub content_height_medium: Option<i32>,
  pub content_height_large: Option<i32>,
  pub content_type_small: Option<String>,
  pub content_type_medium: Option<String>,
  pub content_type_large: Option<String>,
  pub content_image_storage_ref: String,
  pub visibility: AccessType,
  pub created_at: chrono::DateTime<chrono::Utc>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
  pub deletion_scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Post {
  pub async fn fetch_by_user(
    user_id: &str,
    visibilities: &Vec<AccessType>,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<Post>, Error> {
    // There's no way yet to bind a Vec<T> in sqlx, so we need to use the limited available postgres support by
    // binding a slice of a built-in supported type i.e. Vec<String>.
    let visibilities_strs: Vec<String> = visibilities.iter().map(|item| item.to_string()).collect();

    let post = sqlx::query_as(
      "SELECT p.* FROM posts p INNER JOIN users u ON p.user_id = u.user_id WHERE u.handle = $1 AND visibility = ANY($2) ORDER BY created_at DESC LIMIT $3 OFFSET $4",
    )
    .bind(user_id)
    .bind(&visibilities_strs[..])
    .bind(limit)
    .bind(skip)
    .fetch_all(pool)
    .await?;

    Ok(post)
  }

  pub async fn count_by_user(
    user_id: &str,
    visibilities: &Vec<AccessType>,
    pool: &Pool<Postgres>,
  ) -> Result<i64, Error> {
    // There's no way yet to bind a Vec<T> in sqlx, so we need to use the limited available postgres support by
    // binding a slice of a built-in supported type i.e. Vec<String>.
    let visibilities_strs: Vec<String> = visibilities.iter().map(|item| item.to_string()).collect();
    let count = sqlx::query_scalar("SELECT COUNT(p.*) FROM posts p INNER JOIN users u ON p.user_id = u.user_id WHERE u.handle = $1 AND visibility = ANY($2)")
      .bind(user_id)
      .bind(&visibilities_strs[..])
      .fetch_one(pool)
      .await?;

    Ok(count)
  }
}

impl ActivityConvertible<Image> for Post {
  fn to_activity(&self, base_uri: &str, actor_uri: &str) -> Option<Activity<Image>> {
    let mut image_links: Vec<Link> = vec![];
    match Link::from_post_small(self) {
      Some(link) => image_links.push(link),
      None => {}
    };
    match Link::from_post_medium(self) {
      Some(link) => image_links.push(link),
      None => {}
    };
    match Link::from_post_large(self) {
      Some(link) => image_links.push(link),
      None => {}
    };

    if image_links.is_empty() {
      return None;
    }

    Some(Activity {
      id: format!("{}/{}", &base_uri, &self.uri),
      actor: actor_uri.to_string(),
      published: self.created_at.clone(),
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

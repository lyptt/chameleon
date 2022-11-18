use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::activitypub::{
  activity::Activity, activity_convertible::ActivityConvertible, activity_type::ActivityType, image::Image, link::Link,
};

use super::access_type::AccessType;

#[derive(Deserialize, Serialize, FromRow)]
pub struct PostPub {
  pub post_id: Uuid,
  pub user_id: Uuid,
  pub user_handle: String,
  pub user_fediverse_id: String,
  pub user_avatar_url: Option<String>,
  pub uri: String,
  pub content_md: String,
  pub content_html: String,
  pub content_image_uri_small: Option<String>,
  pub content_image_uri_medium: Option<String>,
  pub content_image_uri_large: Option<String>,
  pub content_width_small: Option<i32>,
  pub content_width_medium: Option<i32>,
  pub content_width_large: Option<i32>,
  pub content_height_small: Option<i32>,
  pub content_height_medium: Option<i32>,
  pub content_height_large: Option<i32>,
  pub content_type_small: Option<String>,
  pub content_type_medium: Option<String>,
  pub content_type_large: Option<String>,
  pub visibility: AccessType,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub content_blurhash: Option<String>,
  pub likes: i64,
  pub liked: Option<bool>,
}

impl PostPub {
  /// Fetches the user's feed from their own perspective, i.e. all of the posts they have submitted
  pub async fn fetch_user_own_feed(
    fediverse_id: &str,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<PostPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_user_own_feed.sql"))
      .bind(fediverse_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(pool)
      .await?;

    Ok(post)
  }

  /// Fetches the count of the posts in the user's feed from their own perspective, i.e. all of the posts they have submitted
  pub async fn count_user_own_feed(fediverse_id: &str, pool: &Pool<Postgres>) -> Result<i64, Error> {
    let count = sqlx::query_scalar(include_str!("../db/count_user_own_feed.sql"))
      .bind(fediverse_id)
      .fetch_one(pool)
      .await?;

    Ok(count)
  }

  /// Fetches the user's federated feed, i.e. what users on any server can see
  pub async fn fetch_user_federated_feed(
    fediverse_id: &str,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<PostPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_user_federated_feed.sql"))
      .bind(fediverse_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(pool)
      .await?;

    Ok(post)
  }

  /// Fetches the count of the user's posts in their federated feed, i.e.
  /// what users on any server can see
  pub async fn count_user_federated_feed(fediverse_id: &str, pool: &Pool<Postgres>) -> Result<i64, Error> {
    let count = sqlx::query_scalar(include_str!("../db/count_user_federated_feed.sql"))
      .bind(fediverse_id)
      .fetch_one(pool)
      .await?;

    Ok(count)
  }

  /// Fetches the user's public feed, i.e. what users that follow this user
  /// can see, or alternatively all the user's public posts
  pub async fn fetch_user_public_feed(
    target_user_fediverse_id: &str,
    own_user_fediverse_id: &str,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<PostPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_user_public_feed.sql"))
      .bind(target_user_fediverse_id)
      .bind(own_user_fediverse_id)
      .bind(limit)
      .bind(skip)
      .fetch_all(pool)
      .await?;

    Ok(post)
  }

  /// Fetches the count of posts in the user's public feed, i.e. what users that follow this
  /// user can see, or alternatively all the user's public posts
  pub async fn count_user_public_feed(
    target_user_fediverse_id: &str,
    own_user_fediverse_id: &str,
    pool: &Pool<Postgres>,
  ) -> Result<i64, Error> {
    let count = sqlx::query_scalar(include_str!("../db/count_user_public_feed.sql"))
      .bind(target_user_fediverse_id)
      .bind(own_user_fediverse_id)
      .fetch_one(pool)
      .await?;

    Ok(count)
  }

  /// Fetches the global federated feed, i.e. what users not signed into this instance can see
  pub async fn fetch_global_federated_feed(
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<PostPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_global_federated_feed.sql"))
      .bind(limit)
      .bind(skip)
      .fetch_all(pool)
      .await?;

    Ok(post)
  }

  /// Fetches the post count for the global federated feed, i.e. what users not signed into this instance can see
  pub async fn count_global_federated_feed(pool: &Pool<Postgres>) -> Result<i64, Error> {
    let count = sqlx::query_scalar(include_str!("../db/count_global_federated_feed.sql"))
      .fetch_one(pool)
      .await?;

    Ok(count)
  }
  /// Fetches the user's feed from their own perspective, i.e. all of the posts they have submitted
  pub async fn fetch_post(post_id: &Uuid, pool: &Pool<Postgres>) -> Result<Option<PostPub>, Error> {
    let post = sqlx::query_as(include_str!("../db/fetch_post.sql"))
      .bind(post_id)
      .fetch_optional(pool)
      .await?;

    Ok(post)
  }
}

impl ActivityConvertible<Image> for PostPub {
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

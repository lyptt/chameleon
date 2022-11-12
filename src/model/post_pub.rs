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
}

impl PostPub {
  /// Fetches the user's feed from their own perspective, i.e. all of the posts they have submitted
  pub async fn fetch_user_own_feed(
    fediverse_id: &str,
    limit: i64,
    skip: i64,
    pool: &Pool<Postgres>,
  ) -> Result<Vec<PostPub>, Error> {
    let post = sqlx::query_as(
      "SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url FROM posts p
      INNER JOIN users u
      ON u.user_id = p.user_id
      INNER JOIN (
        SELECT p.post_id as post_id, p.user_id as user_id from followers f
        INNER JOIN users u1
        ON u1.user_id = f.user_id
        INNER JOIN users u2
        ON u2.user_id = f.following_user_id
        LEFT OUTER JOIN posts p
        ON p.user_id = u1.user_id OR p.user_id = u2.user_id
        WHERE u1.fediverse_id = $1
        AND (
          (p.user_id = u1.user_id AND p.visibility IN ('shadow', 'unlisted', 'private', 'followers_only', 'public_local', 'public_federated'))
          OR (p.user_id = u2.user_id AND p.visibility IN ('followers_only', 'public_local', 'public_federated'))
        )
        ORDER BY p.created_at DESC
      ) AS pu
      ON p.post_id = pu.post_id
      AND u.user_id = pu.user_id
      ORDER BY p.created_at DESC
      LIMIT $2
      OFFSET $3",
    )
    .bind(fediverse_id)
    .bind(limit)
    .bind(skip)
    .fetch_all(pool)
    .await?;

    Ok(post)
  }

  /// Fetches the count of the posts in the user's feed from their own perspective, i.e. all of the posts they have submitted
  pub async fn count_user_own_feed(fediverse_id: &str, pool: &Pool<Postgres>) -> Result<i64, Error> {
    let count = sqlx::query_scalar("SELECT COUNT(DISTINCT p.post_id) FROM posts p
    INNER JOIN users u
    ON u.user_id = p.user_id
    INNER JOIN (
      SELECT p.post_id as post_id, p.user_id as user_id from followers f
      INNER JOIN users u1
      ON u1.user_id = f.user_id
      INNER JOIN users u2
      ON u2.user_id = f.following_user_id
      LEFT OUTER JOIN posts p
      ON p.user_id = u1.user_id OR p.user_id = u2.user_id
      WHERE u1.fediverse_id = $1
      AND (
        (p.user_id = u1.user_id AND p.visibility IN ('shadow', 'unlisted', 'private', 'followers_only', 'public_local', 'public_federated'))
        OR (p.user_id = u2.user_id AND p.visibility IN ('followers_only', 'public_local', 'public_federated'))
      )
      ORDER BY p.created_at DESC
    ) AS pu
    ON p.post_id = pu.post_id
    AND u.user_id = pu.user_id")
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
    let post = sqlx::query_as(
      "SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url from posts p
      INNER JOIN users u
      ON u.user_id = p.user_id
      WHERE u.fediverse_id = $1
      AND p.visibility IN ('public_federated')
      ORDER BY p.created_at DESC
      LIMIT $2
      OFFSET $3",
    )
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
    let count = sqlx::query_scalar(
      "SELECT DISTINCT COUNT(p.*) from posts p
    INNER JOIN users u
    ON u.user_id = p.user_id
    WHERE u.fediverse_id = $1
    AND p.visibility IN ('public_federated')",
    )
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
    let post = sqlx::query_as(
      "SELECT DISTINCT p.*, u1.user_id as user_id, u1.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url from followers f
      INNER JOIN users u1
      ON u1.user_id = f.user_id
      INNER JOIN users u2
      ON u2.user_id = f.following_user_id
      LEFT OUTER JOIN posts p
      ON p.user_id = u1.user_id
      WHERE u1.fediverse_id = $1
      AND (
        (p.visibility IN ('public_local', 'public_federated'))
        OR (u2.fediverse_id = $2 AND p.visibility = 'followers_only')
      )
      ORDER BY p.created_at DESC
      LIMIT $3
      OFFSET $4",
    )
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
    let count = sqlx::query_scalar(
      "SELECT DISTINCT COUNT(p.*) from followers f
      INNER JOIN users u1
      ON u1.user_id = f.user_id
      INNER JOIN users u2
      ON u2.user_id = f.following_user_id
      LEFT OUTER JOIN posts p
      ON p.user_id = u1.user_id
      WHERE u1.fediverse_id = $1
      AND (
        (p.visibility IN ('public_local', 'public_federated'))
        OR (u2.fediverse_id = $2 AND p.visibility = 'followers_only')
      )",
    )
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
    let post = sqlx::query_as(
      "SELECT DISTINCT p.*, u.user_id, u.handle as user_handle, u.fediverse_id as user_fediverse_id, u.avatar_url as user_avatar_url from posts p
      INNER JOIN users u
      ON u.user_id = p.user_id
      WHERE p.visibility IN ('public_local', 'public_federated')
      ORDER BY p.created_at DESC
      LIMIT $1
      OFFSET $2",
    )
    .bind(limit)
    .bind(skip)
    .fetch_all(pool)
    .await?;

    Ok(post)
  }

  /// Fetches the post count for the global federated feed, i.e. what users not signed into this instance can see
  pub async fn count_global_federated_feed(pool: &Pool<Postgres>) -> Result<i64, Error> {
    let count = sqlx::query_scalar(
      "SELECT COUNT(DISTINCT p.*) from posts p
      WHERE p.visibility IN ('public_local', 'public_federated')",
    )
    .fetch_one(pool)
    .await?;

    Ok(count)
  }
}

impl ActivityConvertible<Image> for PostPub {
  fn to_activity(&self, base_uri: &str, actor_uri: &str) -> Option<Activity<Image>> {
    let mut image_links: Vec<Link> = vec![];
    match Link::from_post_pub_small(self) {
      Some(link) => image_links.push(link),
      None => {}
    };
    match Link::from_post_pub_medium(self) {
      Some(link) => image_links.push(link),
      None => {}
    };
    match Link::from_post_pub_large(self) {
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

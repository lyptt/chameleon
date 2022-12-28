use crate::{
  helpers::api::map_db_err,
  logic::LogicErr,
  model::{access_type::AccessType, post::Post, post_event::PostEvent},
};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

use super::{FromRow, FromRows};
#[cfg_attr(test, automock)]
#[async_trait]
pub trait PostRepo {
  async fn fetch_user_own_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of the posts in the user's feed from their own perspective, i.e. all of the posts they have submitted
  async fn count_user_own_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  /// Fetches the user's federated feed, i.e. what users on any server can see
  async fn fetch_user_federated_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of the user's posts in their federated feed, i.e.
  /// what users on any server can see
  async fn count_user_federated_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr>;
  /// Fetches the user's public feed, i.e. what users that follow this user
  /// can see, or alternatively all the user's public posts
  async fn fetch_user_public_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of posts in the user's public feed, i.e. what users that follow this
  /// user can see, or alternatively all the user's public posts
  async fn count_user_public_feed(&self, target_user_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr>;
  /// Fetches the global federated feed, i.e. what users not signed into this instance can see
  async fn fetch_global_federated_feed(&self, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the post count for the global federated feed, i.e. what users not signed into this instance can see
  async fn count_global_federated_feed(&self) -> Result<i64, LogicErr>;
  async fn fetch_by_id(&self, id: &Uuid) -> Result<Post, LogicErr>;
  /// Fetches the specified post from a user's own perspective
  async fn fetch_post(&self, post_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr>;
  async fn fetch_post_from_uri(&self, post_uri: &str, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr>;
  async fn create_post(
    &self,
    user_id: &Uuid,
    content_md: &str,
    content_html: &str,
    visibility: &AccessType,
  ) -> Result<Uuid, LogicErr>;
  async fn create_post_from(&self, post: Post) -> Result<(), LogicErr>;
  async fn user_owns_post(&self, user_id: &Uuid, post_id: &Uuid) -> bool;
  async fn find_optional_by_id(&self, post_id: &Uuid) -> Option<Post>;
  async fn find_optional_by_uri(&self, post_uri: &str) -> Option<Post>;
  async fn update_post_content(&self, post: &Post) -> Result<(), LogicErr>;
  async fn fetch_visibility_by_id(&self, post_id: &Uuid) -> Option<AccessType>;
  async fn fetch_owner_by_id(&self, post_id: &Uuid) -> Option<Uuid>;
  async fn fetch_owner_handle_by_id(&self, post_id: &Uuid) -> Option<String>;
  async fn fetch_post_count(&self) -> i64;
  /// Fetches the user's public feed, i.e. what users that follow this user
  /// can see, or alternatively all the user's public posts
  async fn fetch_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr>;
  /// Fetches the count of posts in the user's public feed, i.e. what users that follow this
  /// user can see, or alternatively all the user's public posts
  async fn count_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
  ) -> Result<i64, LogicErr>;
  async fn delete_post(&self, post_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr>;
  async fn delete_post_from_uri(&self, uri: &str, user_id: &Uuid) -> Result<(), LogicErr>;
}

pub type PostPool = Arc<dyn PostRepo + Send + Sync>;

pub struct DbPostRepo {
  pub db: Pool,
}

#[async_trait]
impl PostRepo for DbPostRepo {
  async fn fetch_user_own_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        include_str!("./sql/fetch_user_own_feed.sql"),
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    PostEvent::from_rows(rows)
  }

  async fn count_user_own_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(include_str!("./sql/count_user_own_feed.sql"), &[&user_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_user_federated_feed(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        include_str!("./sql/fetch_user_federated_feed.sql"),
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    PostEvent::from_rows(rows)
  }

  async fn count_user_federated_feed(&self, user_id: &Uuid) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(include_str!("./sql/count_user_federated_feed.sql"), &[&user_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_user_public_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        include_str!("./sql/fetch_user_public_feed.sql"),
        &[&target_user_id, &own_user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    PostEvent::from_rows(rows)
  }

  async fn count_user_public_feed(&self, target_user_id: &Uuid, own_user_id: &Option<Uuid>) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        include_str!("./sql/count_user_public_feed.sql"),
        &[&target_user_id, &own_user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_global_federated_feed(&self, limit: i64, skip: i64) -> Result<Vec<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(include_str!("./sql/fetch_global_federated_feed.sql"), &[&limit, &skip])
      .await
      .map_err(map_db_err)?;

    PostEvent::from_rows(rows)
  }

  async fn count_global_federated_feed(&self) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(include_str!("./sql/count_global_federated_feed.sql"), &[])
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn fetch_by_id(&self, id: &Uuid) -> Result<Post, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;

    let row = db
      .query_one("SELECT * FROM posts WHERE post_id = $1", &[&id])
      .await
      .map_err(map_db_err)?;

    match Post::from_row(row) {
      Some(post) => Ok(post),
      None => Err(LogicErr::MissingRecord),
    }
  }

  async fn fetch_post(&self, post_id: &Uuid, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;

    let rows = db
      .query(include_str!("./sql/fetch_post.sql"), &[&post_id, &user_id])
      .await
      .map_err(map_db_err)?;

    let mut posts = PostEvent::from_rows(rows)?;

    match posts.len() {
      1 => Ok(Some(posts.remove(0))),
      _ => Ok(None),
    }
  }

  async fn fetch_post_from_uri(&self, post_uri: &str, user_id: &Option<Uuid>) -> Result<Option<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;

    let rows = db
      .query(include_str!("./sql/fetch_post_from_uri.sql"), &[&post_uri, &user_id])
      .await
      .map_err(map_db_err)?;

    let mut posts = PostEvent::from_rows(rows)?;

    match posts.len() {
      1 => Ok(Some(posts.remove(0))),
      _ => Ok(None),
    }
  }

  async fn create_post(
    &self,
    user_id: &Uuid,
    content_md: &str,
    content_html: &str,
    visibility: &AccessType,
  ) -> Result<Uuid, LogicErr> {
    let post_id = Uuid::new_v4();
    let uri = post_id.to_string();

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db.query_one(
      "INSERT INTO posts (post_id, user_id, content_md, content_html, visibility, uri) VALUES ($1, $2, $3, $4, $5, $6) RETURNING post_id",
      &[&post_id, &user_id, &content_md, &content_html, &visibility.to_string(), &uri],
    )
    .await
    .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn create_post_from(&self, post: Post) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "INSERT INTO posts (post_id, user_id, uri, is_external, content_md, content_html, visibility, created_at, updated_at, deletion_scheduled_at) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
      &[
        &post.post_id,
        &post.user_id,
        &post.uri,
        &post.is_external,
        &post.content_md,
        &post.content_html,
        &post.visibility.to_string(),
        &post.created_at,
        &post.updated_at,
        &post.deletion_scheduled_at,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn user_owns_post(&self, user_id: &Uuid, post_id: &Uuid) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };

    let row = match db
      .query_one(
        "SELECT COUNT(*) > 0 FROM posts WHERE user_id = $1 AND post_id = $2",
        &[&user_id, &post_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
  }

  async fn find_optional_by_id(&self, post_id: &Uuid) -> Option<Post> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_opt("SELECT * FROM posts WHERE post_id = $1", &[&post_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(Post::from_row)
  }

  async fn find_optional_by_uri(&self, uri: &str) -> Option<Post> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_opt("SELECT * FROM posts WHERE uri = $1", &[&uri])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(Post::from_row)
  }

  async fn update_post_content(&self, post: &Post) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "UPDATE posts SET content_html = $2, content_md = $3, visibility = $4, created_at = $5, updated_at = $6 WHERE post_id = $1",
      &[
        &post.post_id,
        &post.content_html,
        &post.content_md,
        &post.visibility.to_string(),
        &post.created_at,
        &post.updated_at,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn fetch_visibility_by_id(&self, post_id: &Uuid) -> Option<AccessType> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_opt("SELECT visibility FROM posts WHERE post_id = $1", &[&post_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    match row {
      Some(row) => match AccessType::from_str(row.get(0)) {
        Ok(at) => Some(at),
        Err(_) => None,
      },
      None => None,
    }
  }

  async fn fetch_owner_by_id(&self, post_id: &Uuid) -> Option<Uuid> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_one("SELECT user_id FROM posts WHERE post_id = $1", &[&post_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    Some(row.get(0))
  }

  async fn fetch_owner_handle_by_id(&self, post_id: &Uuid) -> Option<String> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };

    let row = match db
      .query_one(
        "SELECT u.handle FROM posts p INNER JOIN users u ON u.user_id = p.user_id WHERE p.post_id = $1",
        &[&post_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    Some(row.get(0))
  }

  async fn fetch_post_count(&self) -> i64 {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return 0,
    };
    let row = match db
      .query_one("SELECT COUNT(*) FROM posts", &[])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return 0,
    };

    row.get(0)
  }

  async fn fetch_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
    limit: i64,
    skip: i64,
  ) -> Result<Vec<PostEvent>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        include_str!("./sql/fetch_user_public_likes_feed.sql"),
        &[&target_user_id, &own_user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    PostEvent::from_rows(rows)
  }

  async fn count_user_public_likes_feed(
    &self,
    target_user_id: &Uuid,
    own_user_id: &Option<Uuid>,
  ) -> Result<i64, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one(
        include_str!("./sql/count_user_public_likes_feed.sql"),
        &[&target_user_id, &own_user_id],
      )
      .await
      .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn delete_post(&self, post_id: &Uuid, user_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(
      "DELETE FROM posts WHERE post_id = $1 AND user_id = $2",
      &[&post_id, &user_id],
    )
    .await
    .map_err(map_db_err)?;

    Ok(())
  }

  async fn delete_post_from_uri(&self, uri: &str, user_id: &Uuid) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute("DELETE FROM posts WHERE uri = $1 AND user_id = $2", &[&uri, &user_id])
      .await
      .map_err(map_db_err)?;

    Ok(())
  }
}

use super::FromRow;
use crate::{helpers::api::map_db_err, logic::LogicErr, model::user::User};

use async_trait::async_trait;
use deadpool_postgres::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserRepo {
  async fn fetch_by_id(&self, id: &Uuid) -> Result<User, LogicErr>;
  async fn fetch_by_handle(&self, handle: &str) -> Result<Option<User>, LogicErr>;
  async fn fetch_id_by_handle(&self, handle: &str) -> Option<Uuid>;
  async fn fetch_id_by_fediverse_id(&self, fediverse_id: &str) -> Option<Uuid>;
  async fn fetch_by_fediverse_id(&self, fediverse_id: &str) -> Result<Option<User>, LogicErr>;
  async fn fetch_password_hash(&self, handle: &str) -> Result<Option<String>, LogicErr>;
  async fn fetch_fediverse_id_by_handle(&self, fediverse_id: &str) -> Option<String>;
  async fn fetch_user_count(&self) -> i64;
  async fn fetch_followers(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn fetch_following(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr>;
  async fn fetch_followers_count(&self, user_id: &Uuid) -> i64;
  async fn fetch_following_count(&self, user_id: &Uuid) -> i64;
  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<User>;
  async fn create(
    &self,
    handle: &str,
    fediverse_id: &str,
    avatar_url: &Option<String>,
    email: &Option<String>,
    password_hash: &str,
    is_external: bool,
    private_key: &str,
    public_key: &str,
  ) -> Result<Uuid, LogicErr>;
  async fn create_from(&self, user: &User) -> Result<User, LogicErr>;
  async fn update_from(&self, user: &User) -> Result<User, LogicErr>;
  async fn delete_user_from_uri(&self, uri: &str) -> Result<(), LogicErr>;
  async fn user_is_external(&self, user_id: &Uuid) -> bool;
}

pub type UserPool = Arc<dyn UserRepo + Send + Sync>;

pub struct DbUserRepo {
  pub db: Pool,
}

#[async_trait]
impl UserRepo for DbUserRepo {
  async fn fetch_by_id(&self, id: &Uuid) -> Result<User, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_one("SELECT * FROM users WHERE user_id = $1", &[&id])
      .await
      .map_err(map_db_err)?;

    match User::from_row(row) {
      Some(user) => Ok(user),
      None => Err(LogicErr::MissingRecord),
    }
  }

  async fn fetch_by_handle(&self, handle: &str) -> Result<Option<User>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT * FROM users WHERE handle = $1", &[&handle])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(User::from_row))
  }

  async fn fetch_id_by_handle(&self, handle: &str) -> Option<Uuid> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT user_id FROM users WHERE handle = $1", &[&handle])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(|r| r.get(0))
  }

  async fn fetch_id_by_fediverse_id(&self, fediverse_id: &str) -> Option<Uuid> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT user_id FROM users WHERE fediverse_id = $1", &[&fediverse_id])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(|r| r.get(0))
  }

  async fn fetch_by_fediverse_id(&self, fediverse_id: &str) -> Result<Option<User>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT * FROM users WHERE fediverse_id = $1", &[&fediverse_id])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(User::from_row))
  }

  async fn fetch_password_hash(&self, handle: &str) -> Result<Option<String>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db
      .query_opt("SELECT password_hash FROM users WHERE handle = $1", &[&handle])
      .await
      .map_err(map_db_err)?;

    Ok(row.and_then(|r| r.get(0)))
  }

  async fn fetch_fediverse_id_by_handle(&self, handle: &str) -> Option<String> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT fediverse_id FROM users WHERE handle = $1", &[&handle])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(|r| r.get(0))
  }

  async fn fetch_user_count(&self) -> i64 {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return 0,
    };
    let row = match db
      .query_one("SELECT COUNT(*) FROM users", &[])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return 0,
    };

    row.get(0)
  }

  async fn fetch_followers(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        r#"SELECT u.* FROM users u INNER JOIN followers f ON f.user_id = u.user_id
        WHERE f.following_user_id = $1 AND f.user_id != following_user_id LIMIT $2 OFFSET $3"#,
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(User::from_row).collect())
  }

  async fn fetch_following(&self, user_id: &Uuid, limit: i64, skip: i64) -> Result<Vec<User>, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    let rows = db
      .query(
        r#"SELECT u.* FROM users u INNER JOIN followers f ON f.following_user_id = u.user_id
        WHERE f.user_id = $1 AND f.user_id != following_user_id LIMIT $2 OFFSET $3"#,
        &[&user_id, &limit, &skip],
      )
      .await
      .map_err(map_db_err)?;

    Ok(rows.into_iter().flat_map(User::from_row).collect())
  }

  async fn fetch_followers_count(&self, user_id: &Uuid) -> i64 {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return 0,
    };
    let row = match db
      .query_one(
        "SELECT COUNT(*) FROM followers WHERE following_user_id = $1 AND user_id != following_user_id",
        &[&user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return 0,
    };

    row.get(0)
  }

  async fn fetch_following_count(&self, user_id: &Uuid) -> i64 {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return 0,
    };
    let row = match db
      .query_one(
        "SELECT COUNT(*) FROM followers WHERE user_id = $1 AND user_id != following_user_id ",
        &[&user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return 0,
    };

    row.get(0)
  }

  async fn fetch_by_fediverse_uri(&self, fediverse_uri: &str) -> Option<User> {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return None,
    };
    let row = match db
      .query_opt("SELECT * FROM users WHERE handle = $1", &[&fediverse_uri])
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return None,
    };

    row.and_then(User::from_row)
  }

  async fn create(
    &self,
    handle: &str,
    fediverse_id: &str,
    avatar_url: &Option<String>,
    email: &Option<String>,
    password_hash: &str,
    is_external: bool,
    private_key: &str,
    public_key: &str,
  ) -> Result<Uuid, LogicErr> {
    let user_id = Uuid::new_v4();
    let fediverse_uri = format!("/user/{user_id}");

    let db = self.db.get().await.map_err(map_db_err)?;
    let row = db.query_one(r#"INSERT INTO users (user_id, handle, fediverse_id, fediverse_uri, avatar_url, email, password_hash, is_external, private_key, public_key) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING user_id"#,
      &[
        &user_id,
        &handle,
        &fediverse_id,
        &fediverse_uri,
        &avatar_url,
        &email,
        &password_hash,
        &is_external,
        &private_key,
        &public_key,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(row.get(0))
  }

  async fn create_from(&self, user: &User) -> Result<User, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(r#"INSERT INTO users (user_id, handle, fediverse_id, fediverse_uri, avatar_url, email, password_hash, is_external, 
      url_1, url_2, url_3, url_4, url_5, url_1_title, url_2_title, url_3_title, url_4_title, url_5_title, intro_md, intro_html, private_key, public_key, 
      ext_apub_followers_uri, ext_apub_following_uri, ext_apub_inbox_uri, ext_apub_outbox_uri) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26) RETURNING user_id"#,
      &[
        &user.user_id,
        &user.handle,
        &user.fediverse_id,
        &user.fediverse_uri,
        &user.avatar_url,
        &user.email,
        &user.password_hash,
        &user.is_external,
        &user.url_1,
        &user.url_2,
        &user.url_3,
        &user.url_4,
        &user.url_5,
        &user.url_1_title,
        &user.url_2_title,
        &user.url_3_title,
        &user.url_4_title,
        &user.url_5_title,
        &user.intro_md,
        &user.intro_html,
        &user.private_key,
        &user.public_key,
        &user.ext_apub_followers_uri,
        &user.ext_apub_following_uri,
        &user.ext_apub_inbox_uri,
        &user.ext_apub_outbox_uri,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(user.to_owned())
  }

  async fn update_from(&self, user: &User) -> Result<User, LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute(r#"UPDATE users SET handle = $2, fediverse_id = $3, fediverse_uri = $4, avatar_url = $5, email = $6, password_hash = $7, is_external = $8, 
    url_1 = $9, url_2 = $10, url_3 = $11, url_4 = $12, url_5 = $13, url_1_title = $14, url_2_title = $15, url_3_title = $16, url_4_title = $17, url_5_title = $18, intro_md = $19, intro_html = $20, private_key = $21, public_key = $22, 
    ext_apub_followers_uri = $23, ext_apub_following_uri = $24, ext_apub_inbox_uri = $25, ext_apub_outbox_uri = $26 WHERE user_id = $1"#,
      &[
        &user.user_id,
        &user.handle,
        &user.fediverse_id,
        &user.fediverse_uri,
        &user.avatar_url,
        &user.email,
        &user.password_hash,
        &user.is_external,
        &user.url_1,
        &user.url_2,
        &user.url_3,
        &user.url_4,
        &user.url_5,
        &user.url_1_title,
        &user.url_2_title,
        &user.url_3_title,
        &user.url_4_title,
        &user.url_5_title,
        &user.intro_md,
        &user.intro_html,
        &user.private_key,
        &user.public_key,
        &user.ext_apub_followers_uri,
        &user.ext_apub_following_uri,
        &user.ext_apub_inbox_uri,
        &user.ext_apub_outbox_uri,
      ],
    )
    .await
    .map_err(map_db_err)?;

    Ok(user.to_owned())
  }

  async fn delete_user_from_uri(&self, uri: &str) -> Result<(), LogicErr> {
    let db = self.db.get().await.map_err(map_db_err)?;
    db.execute("DELETE FROM users WHERE fediverse_uri = $1", &[&uri])
      .await
      .map_err(map_db_err)?;

    Ok(())
  }

  async fn user_is_external(&self, user_id: &Uuid) -> bool {
    let db = match self.db.get().await.map_err(map_db_err) {
      Ok(db) => db,
      Err(_) => return false,
    };
    let row = match db
      .query_one(
        "SELECT COUNT(*) >= 1 FROM users WHERE user_id = $1 is_external = TRUE",
        &[&user_id],
      )
      .await
      .map_err(map_db_err)
    {
      Ok(row) => row,
      Err(_) => return false,
    };

    row.get(0)
  }
}

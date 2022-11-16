use actix_web::web;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;

use crate::{
  helpers::api::{map_db_err, map_ext_err},
  model::user::User,
  net::jwt::JwtFactory,
};

use super::LogicErr;

pub async fn get_user_by_id(handle: &String, db: &web::Data<PgPool>) -> Result<Option<User>, LogicErr> {
  User::fetch_by_handle(handle, db).await.map_err(map_db_err)
}

pub async fn get_user_by_fediverse_id(fediverse_id: &String, db: &web::Data<PgPool>) -> Result<Option<User>, LogicErr> {
  User::fetch_by_fediverse_id(fediverse_id, db).await.map_err(map_db_err)
}

pub async fn authorize_user(username: &str, password: &str, db: &web::Data<PgPool>) -> Result<String, LogicErr> {
  let current_hash = match User::fetch_password_hash(username, db).await.map_err(map_ext_err)? {
    Some(hash) => hash,
    None => return Err(LogicErr::UnauthorizedError),
  };

  let hash = match PasswordHash::new(&current_hash) {
    Ok(hash) => hash,
    Err(err) => {
      println!("{}", err);
      return Err(LogicErr::InternalError("Invalid password hash".to_string()));
    }
  };

  if Argon2::default().verify_password(password.as_bytes(), &hash).is_err() {
    return Err(LogicErr::UnauthorizedError);
  }

  JwtFactory::generate_jwt_short_lived(username)
}

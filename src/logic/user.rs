use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::{
  db::user_repository::UserPool,
  helpers::api::{map_db_err, map_ext_err},
  model::user::User,
  net::jwt::JwtFactory,
};

use super::LogicErr;

pub async fn get_user_by_id(handle: &str, users: &UserPool) -> Result<Option<User>, LogicErr> {
  users.fetch_by_handle(handle).await.map_err(map_db_err)
}

pub async fn get_user_by_fediverse_id(fediverse_id: &str, users: &UserPool) -> Result<Option<User>, LogicErr> {
  users.fetch_by_fediverse_id(fediverse_id).await.map_err(map_db_err)
}

pub async fn authorize_user(username: &str, password: &str, users: &UserPool) -> Result<String, LogicErr> {
  let current_hash = match users.fetch_password_hash(username).await.map_err(map_ext_err)? {
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

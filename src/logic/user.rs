use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use gravatar::{Gravatar, Rating};
use rsa::{
  pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
  pkcs8::LineEnding,
  rand_core::OsRng,
  RsaPrivateKey, RsaPublicKey,
};

use crate::{db::user_repository::UserPool, model::user::User, net::jwt::JwtFactory, settings::SETTINGS};

use super::LogicErr;

pub async fn get_user_by_handle(handle: &str, users: &UserPool) -> Result<Option<User>, LogicErr> {
  users.fetch_by_handle(handle).await
}

pub async fn get_user_by_fediverse_id(fediverse_id: &str, users: &UserPool) -> Result<Option<User>, LogicErr> {
  users.fetch_by_fediverse_id(fediverse_id).await
}

pub async fn authorize_user(username: &str, password: &str, users: &UserPool) -> Result<String, LogicErr> {
  let current_hash = match users.fetch_password_hash(username).await? {
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

pub async fn register_user(
  username: &str,
  password: &str,
  email: &Option<String>,
  users: &UserPool,
) -> Result<String, LogicErr> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();

  let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
    Ok(h) => h,
    Err(err) => return Err(LogicErr::InternalError(err.to_string())),
  }
  .to_string();

  let fediverse_id = format!("acct:{}@{}", username, SETTINGS.server.fqdn);
  let fediverse_uri = format!("/users/{username}");

  let avatar_url = Some(
    Gravatar::new(&email.clone().unwrap_or_else(|| fediverse_id.clone()))
      .set_size(Some(512))
      .set_rating(Some(Rating::Pg))
      .image_url()
      .to_string(),
  );

  let mut rng = rand::thread_rng();
  let bits = 2048;
  let priv_key = match RsaPrivateKey::new(&mut rng, bits) {
    Ok(key) => key,
    Err(err) => return Err(LogicErr::InternalError(err.to_string())),
  };
  let pub_key = RsaPublicKey::from(&priv_key);

  let priv_key = match priv_key.to_pkcs1_pem(LineEnding::LF) {
    Ok(key) => key.to_string(),
    Err(err) => return Err(LogicErr::InternalError(err.to_string())),
  };

  let pub_key = match pub_key.to_pkcs1_pem(LineEnding::LF) {
    Ok(key) => key.to_string(),
    Err(err) => return Err(LogicErr::InternalError(err.to_string())),
  };

  match users
    .create(
      username,
      &fediverse_id,
      &fediverse_uri,
      &avatar_url,
      email,
      &password_hash,
      false,
      &priv_key,
      &pub_key,
    )
    .await
  {
    Ok(_) => {}
    Err(err) => return Err(LogicErr::DbError(err.to_string())),
  };

  JwtFactory::generate_jwt_short_lived(username)
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use mockall::predicate::*;

  use crate::{
    db::user_repository::{MockUserRepo, UserPool},
    logic::{
      user::{authorize_user, get_user_by_fediverse_id, get_user_by_handle},
      LogicErr,
    },
  };

  #[async_std::test]
  async fn test_get_user_by_id_rejects_for_missing_user() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("handle"))
      .return_const(Err(LogicErr::MissingRecord));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(get_user_by_handle("handle", &users).await, Err(LogicErr::MissingRecord));
  }

  #[async_std::test]
  async fn test_get_user_by_id_succeeds() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_by_handle()
      .times(1)
      .with(eq("handle"))
      .return_const(Ok(None));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(get_user_by_handle("handle", &users).await, Ok(None));
  }

  #[async_std::test]
  async fn test_get_user_by_fediverse_id_rejects_for_missing_user() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_by_fediverse_id()
      .times(1)
      .with(eq("handle"))
      .return_const(Err(LogicErr::MissingRecord));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(
      get_user_by_fediverse_id("handle", &users).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_get_user_by_fediverse_id_succeeds() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_by_fediverse_id()
      .times(1)
      .with(eq("handle"))
      .return_const(Ok(None));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(get_user_by_fediverse_id("handle", &users).await, Ok(None));
  }

  #[async_std::test]
  async fn test_authorize_user_rejects_missing_user() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_password_hash()
      .times(1)
      .with(eq("handle"))
      .return_const(Err(LogicErr::MissingRecord));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(
      authorize_user("handle", "test", &users).await,
      Err(LogicErr::MissingRecord)
    );
  }

  #[async_std::test]
  async fn test_authorize_user_rejects_missing_hash() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_password_hash()
      .times(1)
      .with(eq("handle"))
      .return_const(Ok(None));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(
      authorize_user("handle", "test", &users).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_authorize_user_rejects_invalid_password() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_password_hash()
      .times(1)
      .with(eq("handle"))
      .return_const(Ok(Some(
        "$argon2id$v=19$m=4096,t=3,p=1$AAAAAAAAAAA$AZy4qHIzKBofdyGe6tO7fhh3Xl+3356Mi9SDONRcREE".to_string(),
      )));

    let users: UserPool = Arc::new(user_repo);

    assert_eq!(
      authorize_user("handle", "test___", &users).await,
      Err(LogicErr::UnauthorizedError)
    );
  }

  #[async_std::test]
  async fn test_authorize_user_succeeds() {
    let mut user_repo = MockUserRepo::new();
    user_repo
      .expect_fetch_password_hash()
      .times(1)
      .with(eq("handle"))
      .return_const(Ok(Some(
        "$argon2id$v=19$m=4096,t=3,p=1$AAAAAAAAAAA$AZy4qHIzKBofdyGe6tO7fhh3Xl+3356Mi9SDONRcREE".to_string(),
      )));

    let users: UserPool = Arc::new(user_repo);

    assert!(authorize_user("handle", "test", &users).await.is_ok());
  }
}

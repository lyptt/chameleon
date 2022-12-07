use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::{db::user_repository::UserPool, model::user::User, net::jwt::JwtFactory};

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

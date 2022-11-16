use crate::{
  model::session::Session,
  net::jwt::{JwtContext, JwtContextProps},
};

use actix_web::{web, HttpResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn assert_auth(jwt: &web::ReqData<JwtContext>, db: &Pool<Postgres>) -> Result<(), HttpResponse> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  match Session::query_session_exists(&sid, db).await {
    true => Ok(()),
    false => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub async fn require_auth(
  jwt: &web::ReqData<JwtContext>,
  db: &Pool<Postgres>,
) -> Result<JwtContextProps, HttpResponse> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  match Session::query_session_exists(&sid, db).await {
    true => Ok(props),
    false => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub async fn query_auth(jwt: &web::ReqData<JwtContext>, db: &Pool<Postgres>) -> Option<JwtContextProps> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return None,
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return None,
  };

  match Session::query_session_exists(&sid, db).await {
    true => Some(props),
    false => None,
  }
}

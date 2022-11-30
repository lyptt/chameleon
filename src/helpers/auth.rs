use crate::{
  db::session_repository::SessionPool,
  net::jwt::{JwtContext, JwtContextProps},
};

use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn assert_auth(jwt: &web::ReqData<JwtContext>, sessions: &SessionPool) -> Result<(), HttpResponse> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  match sessions.query_session_exists(&sid).await {
    true => Ok(()),
    false => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub async fn require_auth(
  jwt: &web::ReqData<JwtContext>,
  sessions: &SessionPool,
) -> Result<JwtContextProps, HttpResponse> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return Err(HttpResponse::Unauthorized().finish()),
  };

  match sessions.query_session_exists(&sid).await {
    true => Ok(props),
    false => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub async fn query_auth(jwt: &web::ReqData<JwtContext>, sessions: &SessionPool) -> Option<JwtContextProps> {
  let props = match (**jwt).clone() {
    JwtContext::Valid(props) => props,
    JwtContext::Invalid(_) => return None,
  };

  let sid = match Uuid::parse_str(&props.sid) {
    Ok(sid) => sid,
    Err(_) => return None,
  };

  match sessions.query_session_exists(&sid).await {
    true => Some(props),
    false => None,
  }
}

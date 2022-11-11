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

  match Session::query_session_exists(&sid, &db).await {
    true => return Ok(()),
    false => return Err(HttpResponse::Unauthorized().finish()),
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

  match Session::query_session_exists(&sid, &db).await {
    true => return Ok(props),
    false => return Err(HttpResponse::Unauthorized().finish()),
  }
}

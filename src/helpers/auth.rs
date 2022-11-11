use crate::net::jwt::{JwtContext, JwtContextProps};

use actix_web::{web, HttpResponse};

pub fn assert_auth(jwt: &web::ReqData<JwtContext>) -> Result<(), HttpResponse> {
  match (**jwt).clone() {
    JwtContext::Valid(_) => Ok(()),
    JwtContext::Invalid(_) => Err(HttpResponse::Unauthorized().finish()),
  }
}

pub fn require_auth(jwt: &web::ReqData<JwtContext>) -> Result<JwtContextProps, HttpResponse> {
  match (**jwt).clone() {
    JwtContext::Valid(props) => Ok(props),
    JwtContext::Invalid(_) => Err(HttpResponse::Unauthorized().finish()),
  }
}

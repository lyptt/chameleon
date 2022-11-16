use futures_util::future::LocalBoxFuture;
use std::{
  future::{ready, Ready},
  rc::Rc,
};

use actix_web::{
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  HttpMessage,
};

use super::jwt_session_inner::JwtSessionInner;

pub struct JwtSession(Rc<JwtSessionInner>);

impl JwtSession {
  pub fn default() -> Self {
    JwtSession(Rc::new(JwtSessionInner::new()))
  }
}

impl<S, B> Transform<S, ServiceRequest> for JwtSession
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = actix_web::Error;
  type InitError = ();
  type Transform = JwtSessionMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(JwtSessionMiddleware {
      service,
      inner: self.0.clone(),
    }))
  }
}

pub struct JwtSessionMiddleware<S> {
  service: S,
  inner: Rc<JwtSessionInner>,
}

impl<S, B> Service<ServiceRequest> for JwtSessionMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = actix_web::Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    req
      .extensions_mut()
      .insert(self.inner.parse_jwt(req.headers().get("authorization")));

    let fut = self.service.call(req);
    Box::pin(async move { fut.await })
  }
}

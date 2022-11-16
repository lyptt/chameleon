use actix_web::http::header::HeaderValue;
use chrono::{DateTime, NaiveDateTime, Utc};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use super::jwt::{JwtClaims, JwtContext, JwtContextProps};
use crate::settings::SETTINGS;

pub struct JwtSessionInner {
  decoding_key: DecodingKey,
}

impl From<JwtClaims> for JwtContextProps {
  fn from(claims: JwtClaims) -> Self {
    Self {
      sub: claims.sub,
      iss: claims.iss,
      exp: DateTime::from_utc(
        NaiveDateTime::from_timestamp_opt(claims.exp, 0).expect("Invalid or out of range datetime"),
        Utc,
      ),
      nbf: DateTime::from_utc(
        NaiveDateTime::from_timestamp_opt(claims.nbf, 0).expect("Invalid or out of range datetime"),
        Utc,
      ),
      iat: DateTime::from_utc(
        NaiveDateTime::from_timestamp_opt(claims.iat, 0).expect("Invalid or out of range datetime"),
        Utc,
      ),
      sid: claims.sid,
      uid: claims.uid,
    }
  }
}

impl JwtSessionInner {
  pub fn new() -> Self {
    JwtSessionInner {
      decoding_key: DecodingKey::from_secret(SETTINGS.server.jwt_secret.as_bytes()),
    }
  }

  pub fn parse_jwt(&self, authorization_header: Option<&HeaderValue>) -> JwtContext {
    let authorization_header_value = match authorization_header {
      Some(header) => match header.to_str() {
        Ok(header) => header,
        Err(_) => return JwtContext::Invalid(None),
      },
      None => return JwtContext::Invalid(None),
    };

    if authorization_header_value.is_empty() || !authorization_header_value.starts_with("Bearer") {
      return JwtContext::Invalid(None);
    }

    let raw_jwt_split = authorization_header_value.split(' ');
    let raw_jwt_components = raw_jwt_split.collect::<Vec<&str>>();

    if raw_jwt_components.len() != 2 {
      return JwtContext::Invalid(None);
    }

    let raw_jwt = raw_jwt_components[1];

    let token = match decode::<JwtClaims>(raw_jwt, &self.decoding_key, &Validation::new(Algorithm::HS512)) {
      Ok(token) => token,
      Err(err) => return JwtContext::Invalid(Some(err.to_string())),
    };

    JwtContext::Valid(token.claims.into())
  }
}

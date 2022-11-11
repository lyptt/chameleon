use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum::Display;

use crate::{helpers::handlers::map_ext_err, logic::LogicErr, model::user::User, settings::SETTINGS};

use super::jwt_session_err::JwtSessionErr;

pub struct JwtSessionToken {
  pub access_token: String,
  pub refresh_token: String,
  pub access_expiry: DateTime<Utc>,
  pub refresh_expiry: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct JwtContextProps {
  pub sub: String,
  pub iss: String,
  pub exp: DateTime<Utc>,
  pub nbf: DateTime<Utc>,
  pub iat: DateTime<Utc>,
}

#[derive(Debug, Display, Clone)]
pub enum JwtContext {
  Valid(JwtContextProps),
  Invalid(Option<String>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtClaims {
  pub sub: String,
  pub iss: String,
  pub exp: i64,
  pub nbf: i64,
  pub iat: i64,
}

lazy_static! {
  pub static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_secret(SETTINGS.server.jwt_secret.as_bytes());
  pub static ref JWT_DECODING_KEY: DecodingKey = DecodingKey::from_secret(SETTINGS.server.jwt_secret.as_bytes());
}

pub struct JwtFactory {}

impl JwtFactory {
  pub fn generate_jwt_short_lived(subject: &str) -> Result<String, LogicErr> {
    let now = chrono::offset::Utc::now();

    let claims = JwtClaims {
      sub: subject.to_string(),
      iss: SETTINGS.server.fqdn.clone(),
      exp: (now + chrono::Duration::seconds(30)).timestamp(),
      nbf: (now - chrono::Duration::seconds(30)).timestamp(),
      iat: now.timestamp(),
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &JWT_ENCODING_KEY).map_err(map_ext_err)
  }

  pub fn generate_jwt_long_lived(user: &User) -> Result<JwtSessionToken, JwtSessionErr> {
    if user.is_external {
      // A user must sign into their home instance, not ours
      return Err(JwtSessionErr::InvalidDataErr);
    }

    let now = chrono::offset::Utc::now();

    let mut refresh_token_hasher = Sha256::new();
    let refresh_token_data = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    refresh_token_hasher.update(refresh_token_data.as_bytes());
    let refresh_token = hex::encode(refresh_token_hasher.finalize());

    // TODO: Make expiry time spans and nbf clock skew parameters configurable
    let access_expiry = now + chrono::Duration::days(7);
    let refresh_expiry = now + chrono::Duration::days(30);

    let claims = JwtClaims {
      sub: user.fediverse_id.clone(),
      iss: SETTINGS.server.fqdn.clone(),
      exp: access_expiry.timestamp(),
      nbf: (now - chrono::Duration::seconds(30)).timestamp(),
      iat: now.timestamp(),
    };

    let header = Header::new(Algorithm::HS512);
    let access_token = encode(&header, &claims, &JWT_ENCODING_KEY).map_err(|e| JwtSessionErr::JwtError(e))?;

    Ok(JwtSessionToken {
      access_token,
      refresh_token,
      access_expiry,
      refresh_expiry,
    })
  }

  pub fn parse_jwt_props(jwt: &str) -> Option<JwtClaims> {
    let token = match decode::<JwtClaims>(jwt, &JWT_DECODING_KEY, &Validation::new(Algorithm::HS512)) {
      Ok(token) => token,
      Err(_) => return None,
    };

    Some(token.claims)
  }
}

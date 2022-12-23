use actix_web::{
  guard::{Guard, GuardContext},
  http::header::HeaderMap,
};
use mediatype::MediaTypeList;
use serde::Serialize;

pub const ACTIVITY_JSON_CONTENT_TYPE: &str = "application/activity+json";

pub struct ActivityPubHeaderGuard;

impl ActivityPubHeaderGuard {
  pub(self) fn check_headers(&self, headers: &HeaderMap) -> bool {
    let accept = match headers.get("accept") {
      Some(accept) => accept,
      None => return false,
    };

    let accept_value = match accept.to_str() {
      Ok(val) => val,
      Err(_) => return false,
    };

    let mut accepts = MediaTypeList::new(accept_value);

    accepts.any(|accept| {
      let accept = match accept {
        Ok(accept) => accept,
        Err(err) => {
          println!("{}", err);
          return false;
        }
      };

      accept.ty == "application"
        && (accept.subty == "activity" || accept.subty == "ld")
        && accept.suffix.is_some()
        && accept.suffix.unwrap().as_str() == "json"
    })
  }
}

impl Guard for ActivityPubHeaderGuard {
  fn check(&self, ctx: &GuardContext<'_>) -> bool {
    self.check_headers(&ctx.head().headers)
  }
}

pub const ACTIVITYPUB_ACCEPT_GUARD: ActivityPubHeaderGuard = ActivityPubHeaderGuard {};

#[derive(Serialize, Debug)]
pub struct ApiError {
  pub code: u16,
  pub reason: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cause: Option<String>,
}

#[cfg(test)]
mod tests {
  use actix_web::http::header::HeaderMap;
  use http::{header::HeaderName, HeaderValue};

  use crate::helpers::types::ACTIVITYPUB_ACCEPT_GUARD;

  const ACTIVITY_JSON_CONTENT_TYPE: &[u8] = b"application/activity+json";
  const ACTIVITY_JSON_JSON_CONTENT_TYPE: &[u8] = b"application/activity+json, application/json";
  const ACTIVITY_JSON_LD_JSON_CONTENT_TYPE: &[u8] = b"application/activity+json, application/ld+json";
  const ACTIVITY_LD_JSON_CONTENT_TYPE: &[u8] =
    b"application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";

  #[test]
  pub fn test_apub_guard_denies_plain_json() {
    let mut headers = HeaderMap::new();
    headers.append(
      HeaderName::from_lowercase(b"accept").unwrap(),
      HeaderValue::from_bytes(b"application/json").unwrap(),
    );
    assert!(!ACTIVITYPUB_ACCEPT_GUARD.check_headers(&headers));
  }

  #[test]
  pub fn test_apub_guard_accepts_activity_json() {
    let mut headers = HeaderMap::new();
    headers.append(
      HeaderName::from_lowercase(b"accept").unwrap(),
      HeaderValue::from_bytes(ACTIVITY_JSON_CONTENT_TYPE).unwrap(),
    );
    assert!(ACTIVITYPUB_ACCEPT_GUARD.check_headers(&headers));
  }

  #[test]
  pub fn test_apub_guard_accepts_activity_json_json() {
    let mut headers = HeaderMap::new();
    headers.append(
      HeaderName::from_lowercase(b"accept").unwrap(),
      HeaderValue::from_bytes(ACTIVITY_JSON_JSON_CONTENT_TYPE).unwrap(),
    );
    assert!(ACTIVITYPUB_ACCEPT_GUARD.check_headers(&headers));
  }

  #[test]
  pub fn test_apub_guard_accepts_json_ld_json() {
    let mut headers = HeaderMap::new();
    headers.append(
      HeaderName::from_lowercase(b"accept").unwrap(),
      HeaderValue::from_bytes(ACTIVITY_JSON_LD_JSON_CONTENT_TYPE).unwrap(),
    );
    assert!(ACTIVITYPUB_ACCEPT_GUARD.check_headers(&headers));
  }

  #[test]
  pub fn test_apub_guard_accepts_ld_json() {
    let mut headers = HeaderMap::new();
    headers.append(
      HeaderName::from_lowercase(b"accept").unwrap(),
      HeaderValue::from_bytes(ACTIVITY_LD_JSON_CONTENT_TYPE).unwrap(),
    );
    assert!(ACTIVITYPUB_ACCEPT_GUARD.check_headers(&headers));
  }
}

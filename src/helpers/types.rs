use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
  pub static ref ACTIVITY_JSON_CONTENT_TYPE: &'static str = "application/activity+json";
  pub static ref ACTIVITY_LD_JSON_CONTENT_TYPE: &'static str =
    "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";
}

#[derive(Serialize, Debug)]
pub struct ApiError {
  pub code: u16,
  pub reason: String,
  pub cause: Option<String>,
}

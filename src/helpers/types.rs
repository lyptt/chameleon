use serde::Serialize;

pub const ACTIVITY_JSON_CONTENT_TYPE: &str = "application/activity+json";
pub const ACTIVITY_LD_JSON_CONTENT_TYPE: &str =
  "application/ld+json; profile=\"https://www.w3.org/ns/activitystreams\"";

#[derive(Serialize, Debug)]
pub struct ApiError {
  pub code: u16,
  pub reason: String,
  pub cause: Option<String>,
}

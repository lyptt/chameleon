use serde::{Deserialize, Serialize};

use crate::settings::SETTINGS;

#[derive(Deserialize, Serialize, Clone)]
pub struct WebfingerRecordLink {
  pub rel: String,
  #[serde(rename = "type")]
  pub link_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub href: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template: Option<String>,
}

impl WebfingerRecordLink {
  pub fn build_self_uri(handle: &Option<String>) -> String {
    format!(
      "{}/users/{}",
      SETTINGS.server.api_fqdn,
      handle.clone().unwrap_or_else(|| "<unknown>".to_string())
    )
  }

  pub fn build_self_link(handle: &Option<String>) -> WebfingerRecordLink {
    WebfingerRecordLink {
      rel: "self".to_string(),
      link_type: "application/activity+json".to_string(),
      href: Some(format!(
        "{}/users/{}",
        SETTINGS.server.api_fqdn,
        handle.clone().unwrap_or_else(|| "<unknown>".to_string())
      )),
      template: None,
    }
  }

  pub fn build_feed_link(handle: &Option<String>) -> WebfingerRecordLink {
    WebfingerRecordLink {
      rel: "feed".to_string(),
      link_type: "application/activity+json".to_string(),
      href: Some(format!(
        "{}/users/{}/feed",
        SETTINGS.server.api_fqdn,
        handle.clone().unwrap_or_else(|| "<unknown>".to_string())
      )),
      template: None,
    }
  }

  pub fn build_profile_page_link(handle: &Option<String>) -> WebfingerRecordLink {
    WebfingerRecordLink {
      rel: "http://webfinger.net/rel/profile-page".to_string(),
      link_type: "text/html".to_string(),
      href: Some(format!(
        "{}/users/{}",
        SETTINGS.server.fqdn,
        handle.clone().unwrap_or_else(|| "<unknown>".to_string())
      )),
      template: None,
    }
  }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WebfingerRecord {
  pub subject: String,
  pub links: Vec<WebfingerRecordLink>,
  pub aliases: Option<Vec<String>>,
}

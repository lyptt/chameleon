use serde::{Deserialize, Serialize};

use crate::settings::SETTINGS;

#[derive(Deserialize, Serialize, Clone)]
pub struct WebfingerRecordLink {
  pub rel: String,
  #[serde(rename = "type")]
  pub link_type: String,
  pub href: Option<String>,
  pub template: Option<String>,
}

impl WebfingerRecordLink {
  pub fn build_self_uri(handle: &String) -> String {
    format!("{}/users/{}", SETTINGS.server.api_fqdn, handle.clone())
  }

  pub fn build_self_link(handle: &String) -> WebfingerRecordLink {
    WebfingerRecordLink {
      rel: "self".to_string(),
      link_type: "application/activity+json".to_string(),
      href: Some(format!("{}/users/{}", SETTINGS.server.api_fqdn, handle.clone())),
      template: None,
    }
  }

  pub fn build_profile_page_link(handle: &String) -> WebfingerRecordLink {
    WebfingerRecordLink {
      rel: "http://webfinger.net/rel/profile-page".to_string(),
      link_type: "text/html".to_string(),
      href: Some(format!("{}/feed/{}", SETTINGS.server.fqdn, handle.clone())),
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

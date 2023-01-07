use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::{
  activitypub::{
    activity_convertible::ActivityConvertible, actor::ActorProps, key::KeyProps, object::Object, orbit::OrbitProps,
    rdf_string::RdfString, reference::Reference,
  },
  db::FromRow,
  helpers::{api::relative_cdn_to_absolute_cdn_uri, types::RELATIVE_API_ROOT_FQDN},
  settings::SETTINGS,
};

use super::webfinger::{WebfingerRecord, WebfingerRecordLink};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Orbit {
  pub orbit_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub shortcode: String,
  pub name: String,
  pub description_md: String,
  pub description_html: String,
  pub avatar_uri: Option<String>,
  pub banner_uri: Option<String>,
  pub uri: String,
  pub fediverse_uri: String,
  pub fediverse_id: String,
  pub private_key: String,
  pub public_key: String,
  pub is_external: bool,
  pub ext_apub_inbox_uri: Option<String>,
  pub ext_apub_outbox_uri: Option<String>,
  pub ext_apub_followers_uri: Option<String>,
}

impl Orbit {
  pub fn to_webfinger(&self) -> WebfingerRecord {
    WebfingerRecord {
      aliases: Some(vec![WebfingerRecordLink::build_orbit_self_uri(&self.orbit_id)]),
      subject: format!("group:{}@{}", self.shortcode, *RELATIVE_API_ROOT_FQDN),
      links: [
        WebfingerRecordLink::build_orbit_self_link(&self.orbit_id),
        WebfingerRecordLink::build_orbit_page_link(&self.shortcode),
        WebfingerRecordLink::build_orbit_feed_link(&self.orbit_id),
      ]
      .into(),
    }
  }
}

impl FromRow for Orbit {
  fn from_row(row: Row) -> Option<Self> {
    Some(Orbit {
      orbit_id: row.get("orbit_id"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
      shortcode: row.get("shortcode"),
      name: row.get("name"),
      description_md: row.get("description_md"),
      description_html: row.get("description_html"),
      avatar_uri: row.get("avatar_uri"),
      banner_uri: row.get("banner_uri"),
      uri: row.get("uri"),
      fediverse_uri: row.get("fediverse_uri"),
      fediverse_id: row.get("fediverse_id"),
      private_key: row.get("private_key"),
      public_key: row.get("public_key"),
      is_external: row.get("is_external"),
      ext_apub_inbox_uri: row.get("ext_apub_inbox_uri"),
      ext_apub_outbox_uri: row.get("ext_apub_outbox_uri"),
      ext_apub_followers_uri: row.get("ext_apub_followers_uri"),
    })
  }
}

impl ActivityConvertible for Orbit {
  fn to_object(&self, _actor: &str) -> Option<Object> {
    let id = format!("{}/orbit/{}", SETTINGS.server.api_fqdn, self.orbit_id);
    let public_inbox_uri = format!("{}/federate/activitypub/shared-inbox", SETTINGS.server.api_fqdn);
    let inbox_uri = format!(
      "{}/federate/activitypub/orbit/{}/inbox",
      SETTINGS.server.api_fqdn, &self.orbit_id
    );
    let outbox_uri = format!("{}/orbit/{}/feed", SETTINGS.server.api_fqdn, &self.orbit_id);
    let followers_uri = format!("{}/orbit/{}/members", SETTINGS.server.api_fqdn, &self.orbit_id);
    let icon = self.avatar_uri.clone().map(|avatar_url| {
      Reference::Embedded(Box::new(
        Object::builder()
          .kind(Some("Image".to_string()))
          .media_type(Some("image/jpeg".to_string()))
          .url(Some(Reference::Remote(relative_cdn_to_absolute_cdn_uri(&avatar_url))))
          .build(),
      ))
    });
    let image = self.banner_uri.clone().map(|banner_uri| {
      Reference::Embedded(Box::new(
        Object::builder()
          .kind(Some("Image".to_string()))
          .media_type(Some("image/jpeg".to_string()))
          .url(Some(Reference::Remote(relative_cdn_to_absolute_cdn_uri(&banner_uri))))
          .build(),
      ))
    });
    let mut endpoints = HashMap::new();
    endpoints.insert("sharedInbox".to_string(), serde_json::Value::String(public_inbox_uri));

    let key_props = KeyProps::builder()
      .id(Some(format!("{}#main-key", id)))
      .owner(Some(id.clone()))
      .public_key_pem(Some(self.public_key.clone()))
      .build();

    Some(
      Object::builder()
        .id(Some(id.clone()))
        .kind(Some("Group".to_string()))
        .icon(icon)
        .image(image)
        .url(Some(Reference::Remote(id)))
        .name(Some(self.name.clone()))
        .summary(Some(RdfString::Raw(self.description_html.clone())))
        .actors(Some(
          ActorProps::builder()
            .endpoints(Some(Reference::Map(endpoints)))
            .followers(Some(Reference::Remote(followers_uri)))
            .inbox(Some(Reference::Remote(inbox_uri)))
            .outbox(Some(Reference::Remote(outbox_uri)))
            .preferred_username(Some(self.shortcode.clone()))
            .build(),
        ))
        .orbit(Some(
          OrbitProps::builder()
            .shortcode(Some(self.shortcode.clone()))
            .summary_md(Some(self.description_md.clone()))
            .build(),
        ))
        .key(Some(key_props))
        .build(),
    )
  }
}

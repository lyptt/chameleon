use super::{object::Object, reference::Reference};

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct ActorProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub inbox: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub outbox: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub following: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub followers: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub liked: Option<Reference<Object>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub streams: Option<Reference<Object>>,
  #[serde(rename = "preferredUsername", skip_serializing_if = "Option::is_none")]
  pub preferred_username: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub endpoints: Option<Reference<Object>>,
  #[serde(rename = "proxyUrl", skip_serializing_if = "Option::is_none")]
  pub proxy_url: Option<String>,
  #[serde(rename = "oauthAuthorizationEndpoint", skip_serializing_if = "Option::is_none")]
  pub oauth_authorization_endpoint: Option<String>,
  #[serde(rename = "provideClientKey", skip_serializing_if = "Option::is_none")]
  pub provide_client_key: Option<String>,
  #[serde(rename = "signClientKey", skip_serializing_if = "Option::is_none")]
  pub sign_client_key: Option<String>,
  #[serde(rename = "sharedInbox", skip_serializing_if = "Option::is_none")]
  pub shared_inbox: Option<Reference<Object>>,
}

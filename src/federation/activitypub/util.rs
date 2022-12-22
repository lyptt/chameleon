use std::time::Duration;

use crate::{
  activitypub::{document::ActivityPubDocument, object::Object, reference::Reference},
  model::{access_type::AccessType, user::User},
  settings::SETTINGS,
};

use backoff::{future::retry, ExponentialBackoff};
use lazy_static::lazy_static;
use tokio_stream::{self as stream, StreamExt};

lazy_static! {
  pub static ref BACKOFF_POLICY: ExponentialBackoff = {
    ExponentialBackoff {
      max_elapsed_time: Some(Duration::from_millis(300_000)),
      ..Default::default()
    }
  };
  pub static ref HTTP_CLIENT: reqwest::Client = {
    reqwest::Client::builder()
      .danger_accept_invalid_certs(!SETTINGS.app.verify_external_https_certificates)
      .build()
      .unwrap()
  };
}

pub enum ActivityTarget {
  Unknown,
  UserFollowers(String),
  Post(String),
  PostLikes(String),
}

pub fn activitypub_ref_to_uri(obj_ref: &Reference<Object>) -> Option<String> {
  match obj_ref {
    Reference::Embedded(obj) => activitypub_ref_to_uri_opt(&obj.url),
    Reference::Remote(uri) => Some(uri.to_owned()),
    Reference::Mixed(vals) => vals.iter().find_map(activitypub_ref_to_uri),
    Reference::Map(data) => {
      if let Some(value) = data.get("url") {
        let uri: Result<String, serde_json::Error> = serde_json::from_value(value.to_owned());
        match uri {
          Ok(uri) => Some(uri),
          Err(_) => None,
        }
      } else {
        None
      }
    }
  }
}

pub fn activitypub_ref_to_uri_opt(obj_ref: &Option<Reference<Object>>) -> Option<String> {
  match obj_ref {
    Some(obj_ref) => activitypub_ref_to_uri(obj_ref),
    None => None,
  }
}

pub fn determine_activity_visibility(to: &Option<Reference<Object>>, author: &User) -> Option<AccessType> {
  let objs = match to {
    Some(obj_ref) => match obj_ref {
      Reference::Embedded(_) => return None,
      Reference::Remote(uri) => vec![uri.to_owned()],
      Reference::Mixed(multi) => multi.iter().flat_map(activitypub_ref_to_uri).collect(),
      Reference::Map(_) => return None,
    },
    None => return None,
  };

  if objs.contains(&"https://www.w3.org/ns/activitystreams#Public".to_string()) {
    return Some(AccessType::PublicFederated);
  }

  let followers_uri = author.ext_apub_followers_uri.clone().unwrap_or_default();

  if objs.iter().any(|v| *v == followers_uri) {
    return Some(AccessType::FollowersOnly);
  }

  Some(AccessType::Unlisted)
}

pub async fn fetch_activitypub_object(obj_ref: &str) -> Option<Object> {
  let result: Result<Option<ActivityPubDocument>, reqwest::Error> = retry(BACKOFF_POLICY.clone(), || async {
    Ok(
      HTTP_CLIENT
        .get(obj_ref)
        .header("accept", "application/activity+json")
        .send()
        .await?
        .json()
        .await?,
    )
  })
  .await;

  match result {
    Ok(v) => v.map(|v| v.object),
    Err(_) => None,
  }
}

pub async fn deref_activitypub_ref(obj_ref: &Option<Reference<Object>>) -> Option<Object> {
  match obj_ref {
    Some(a) => match a {
      Reference::Embedded(obj) => Some((**obj).clone()),
      Reference::Remote(uri) => fetch_activitypub_object(uri).await,
      Reference::Mixed(values) => {
        // EDITOR'S NOTE: We could do recursion instead here, but it requires boxing which makes things much slower
        let mut stream = stream::iter(values);
        while let Some(value) = stream.next().await {
          let ret = match value {
            Reference::Embedded(obj) => Some((**obj).clone()),
            Reference::Remote(uri) => fetch_activitypub_object(uri).await,
            Reference::Mixed(_) => None,
            Reference::Map(_) => None,
          };

          if ret.is_some() {
            return ret;
          }
        }

        None
      }
      Reference::Map(_) => None,
    },
    None => None,
  }
}

pub fn determine_activity_target(target: Option<String>) -> ActivityTarget {
  match target {
    Some(target) => {
      if target.ends_with("/followers") {
        let user_uri = target.replace("/followers", "");
        ActivityTarget::UserFollowers(user_uri)
      } else if target.contains("/api/feed") && target.ends_with("/likes") {
        let post_uri = target.replace("/likes", "");
        ActivityTarget::PostLikes(post_uri)
      } else if target.contains("/api/feed") {
        ActivityTarget::Post(target)
      } else {
        ActivityTarget::Unknown
      }
    }
    None => ActivityTarget::Unknown,
  }
}

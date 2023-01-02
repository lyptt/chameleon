use crate::{
  helpers::api::relative_to_absolute_uri,
  model::{event_type::EventType, post_event::PostEvent},
  settings::SETTINGS,
};

use super::{
  activity::ActivityProps,
  activity_convertible::ActivityConvertible,
  activity_type::ActivityType,
  collection::{CollectionPageProps, CollectionProps},
  document::ActivityPubDocument,
  object::Object,
  reference::Reference,
};

pub fn create_activitypub_ordered_collection_page_feed(
  base_uri: &str,
  page: i32,
  page_size: i32,
  total_items: i32,
  posts: Vec<PostEvent>,
) -> ActivityPubDocument {
  let prev = match page {
    0 => None,
    _ => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page - 1,
      page_size
    ))),
  };

  let next = match (page + 1) * page_size > total_items {
    true => None,
    false => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page + 1,
      page_size
    ))),
  };

  let posts = posts
    .into_iter()
    .filter_map(
      |p| match p.to_object(&format!("{}/user/{}", SETTINGS.server.api_fqdn, p.user_id)) {
        Some(post_obj) => {
          let activity = match p.event_type {
            EventType::Boost => ActivityType::Announce,
            _ => ActivityType::Create,
          };

          let id = relative_to_absolute_uri(&p.uri);

          let obj = Object::builder()
            .id(Some(id))
            .kind(Some(activity.to_string()))
            .cc(post_obj.cc.clone())
            .to(post_obj.to.clone())
            .published(post_obj.published)
            .activity(Some(
              ActivityProps::builder()
                .actor(Some(Reference::Remote(relative_to_absolute_uri(&p.user_fediverse_uri))))
                .object(Some(Reference::Embedded(Box::new(post_obj))))
                .build(),
            ))
            .build();
          Some(Reference::Embedded(Box::new(obj)))
        }
        None => None,
      },
    )
    .collect();

  let obj = Object::builder()
    .id(Some(base_uri.to_string()))
    .kind(Some("OrderedCollectionPage".to_string()))
    .collection_page(Some(
      CollectionPageProps::builder()
        .part_of(Some(Reference::Remote(base_uri.to_string())))
        .prev(prev)
        .next(next)
        .build(),
    ))
    .collection(Some(
      CollectionProps::builder()
        .ordered_items(Some(Reference::Mixed(posts)))
        .build(),
    ))
    .build();

  ActivityPubDocument::new(obj)
}

pub fn create_activitypub_ordered_collection_page_specific_feed(
  base_uri: &str,
  page: i32,
  page_size: i32,
  total_items: i32,
  posts: Vec<PostEvent>,
  activity: ActivityType,
) -> ActivityPubDocument {
  let prev = match page {
    0 => None,
    _ => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page - 1,
      page_size
    ))),
  };

  let next = match (page + 1) * page_size > total_items {
    true => None,
    false => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page + 1,
      page_size
    ))),
  };

  let posts = posts
    .into_iter()
    .filter_map(
      |p| match p.to_object(&format!("{}/user/{}", SETTINGS.server.api_fqdn, p.user_id)) {
        Some(post_obj) => {
          let obj = Object::builder()
            .id(Some(format!("{}/{}", &base_uri, &p.uri)))
            .kind(Some(activity.to_string()))
            .cc(post_obj.cc.clone())
            .to(post_obj.to.clone())
            .published(post_obj.published)
            .activity(Some(
              ActivityProps::builder()
                .actor(Some(Reference::Remote(relative_to_absolute_uri(&p.user_fediverse_uri))))
                .object(Some(Reference::Embedded(Box::new(post_obj))))
                .build(),
            ))
            .build();
          Some(Reference::Embedded(Box::new(obj)))
        }
        None => None,
      },
    )
    .collect();

  let obj = Object::builder()
    .id(Some(base_uri.to_string()))
    .kind(Some("OrderedCollectionPage".to_string()))
    .collection_page(Some(
      CollectionPageProps::builder()
        .part_of(Some(Reference::Remote(base_uri.to_string())))
        .prev(prev)
        .next(next)
        .build(),
    ))
    .collection(Some(
      CollectionProps::builder()
        .ordered_items(Some(Reference::Mixed(posts)))
        .build(),
    ))
    .build();

  ActivityPubDocument::new(obj)
}

pub fn create_activitypub_ordered_collection_page<T: ActivityConvertible>(
  base_uri: &str,
  page: i32,
  page_size: i32,
  total_items: i32,
  entities: Vec<T>,
  actor: Option<String>,
) -> ActivityPubDocument {
  let prev = match page {
    0 => None,
    _ => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page - 1,
      page_size
    ))),
  };

  let next = match (page + 1) * page_size > total_items {
    true => None,
    false => Some(Reference::Remote::<Object>(format!(
      "{}?page={}&page_size={}",
      base_uri,
      page + 1,
      page_size
    ))),
  };

  let actor_ref = &actor.unwrap_or_default();

  let entities = entities
    .into_iter()
    .filter_map(|c| {
      c.to_object(actor_ref)
        .map(|user_obj| Reference::Embedded(Box::new(user_obj)))
    })
    .collect();

  let obj = Object::builder()
    .id(Some(base_uri.to_string()))
    .kind(Some("OrderedCollectionPage".to_string()))
    .collection_page(Some(
      CollectionPageProps::builder()
        .part_of(Some(Reference::Remote(base_uri.to_string())))
        .prev(prev)
        .next(next)
        .build(),
    ))
    .collection(Some(
      CollectionProps::builder()
        .ordered_items(Some(Reference::Mixed(entities)))
        .build(),
    ))
    .build();

  ActivityPubDocument::new(obj)
}

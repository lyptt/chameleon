use crate::model::post_event::PostEvent;

use super::{
  activity::ActivityProps,
  activity_convertible::ActivityConvertible,
  collection::{CollectionPageProps, CollectionProps},
  document::ActivityPubDocument,
  object::Object,
  reference::Reference,
};

pub fn create_activitypub_ordered_collection_page_posts(
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
    .map(|p| {
      let post_obj = p.to_object(&p.user_fediverse_id);
      let obj = Object::builder()
        .id(Some(format!("{}/{}", &base_uri, &p.uri)))
        .kind(Some("Create".to_string()))
        .cc(post_obj.cc.clone())
        .to(post_obj.to.clone())
        .published(post_obj.published)
        .activity(Some(
          ActivityProps::builder()
            .actor(Some(Reference::Remote(p.user_fediverse_id)))
            .object(Some(Reference::Embedded(Box::new(post_obj))))
            .build(),
        ))
        .build();
      Reference::Embedded(Box::new(obj))
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
        .ordered_items(Some(Reference::Mixed(posts)))
        .build(),
    ))
    .build();

  ActivityPubDocument::new(obj)
}

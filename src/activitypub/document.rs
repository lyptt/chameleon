use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{logic::LogicErr, settings::SETTINGS};

use super::{
  json_ld::{JsonLdContext, JsonLdContextEntry, JsonLdContextMapEntry},
  object::Object,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
/// Defines the raw type for an ActivityPub document. All data received via ActivityPub is parsed initially into this
/// structure. We then evaluate the JSON-LD context to determine the full key names for every root-level key within
/// the data structure.
///
/// With this in hand, we can then parse the remaining values in the _other_ key, taking into account this metadata.
pub struct RawActivityPubDocument {
  #[serde(rename = "@context")]
  pub context: JsonLdContext,
  #[serde(flatten)]
  pub other: Map<String, Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ActivityPubDocument {
  #[serde(rename = "@context")]
  pub context: JsonLdContext,
  #[serde(flatten)]
  pub object: Object,
  #[serde(skip)]
  pub aliases: Option<HashMap<String, String>>,
}

impl ActivityPubDocument {
  pub fn from(doc: RawActivityPubDocument) -> Result<Self, LogicErr> {
    let alias_pairs = doc.context.get_json_ld_aliases();

    let mut props = doc.other;

    // Transform all root keys into their un-aliased forms ready for deserialization
    for (k, k2) in &alias_pairs {
      if !props.contains_key(k) {
        continue;
      }

      let val = match props.remove(k) {
        Some(val) => val,
        None => continue,
      };

      props.insert(k2.to_string(), val);
    }

    let aliases = alias_pairs.into_iter().collect::<HashMap<String, String>>();
    let object: Object = match serde_json::from_value(serde_json::Value::Object(props)) {
      Ok(obj) => obj,
      Err(_) => return Err(LogicErr::InternalError("Invalid ActivityPub object".to_string())),
    };

    Ok(ActivityPubDocument {
      context: doc.context,
      object,
      aliases: Some(aliases),
    })
  }

  pub fn new(obj: Object) -> Self {
    let mut aliases: HashMap<String, JsonLdContextMapEntry> = HashMap::new();
    aliases.insert(
      "sensitive".to_string(),
      JsonLdContextMapEntry::Alias("as:sensitive".to_string()),
    );
    aliases.insert(
      "shortcode".to_string(),
      JsonLdContextMapEntry::Alias("orbit:shortcode".to_string()),
    );
    aliases.insert(
      "summaryMd".to_string(),
      JsonLdContextMapEntry::Alias("orbit:summaryMd".to_string()),
    );
    aliases.insert(
      "orbit".to_string(),
      JsonLdContextMapEntry::Alias(format!("{}/.well-known/ns", SETTINGS.server.api_root_fqdn)),
    );

    let ctx = JsonLdContext::Multi(vec![
      JsonLdContextEntry::Uri("https://www.w3.org/ns/activitystreams".to_string()),
      JsonLdContextEntry::Uri("https://w3id.org/security/v1".to_string()),
      JsonLdContextEntry::Map(aliases),
    ]);

    ActivityPubDocument {
      context: ctx,
      object: obj,
      aliases: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::{HashMap, HashSet};

  use crate::{
    activitypub::{
      document::ActivityPubDocument,
      json_ld::{JsonLdContext, JsonLdContextEntry, JsonLdContextMapEntry},
      object::Object,
    },
    settings::SETTINGS,
  };

  use super::RawActivityPubDocument;
  use serde::{Deserialize, Serialize};
  use serde_json::Error;

  #[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
  struct TestProps {
    test: i32,
  }

  #[test]
  pub fn raw_object_parses_object_context() {
    // This test performs the rough steps the Object::Into() method goes through when parsing ActivityPub JSON-LD
    // documents.

    // We start out with the raw JSON body, containing a context and a root key with an aliased short name key _test_,
    // resolving to the long name _testy-test_.
    let raw_json = r#"{ "@context": {
      "test": "testy-test"
    },
    "test": 123
  }"#;

    let doc: Result<RawActivityPubDocument, Error> = serde_json::from_str(raw_json);
    assert!(doc.is_ok());

    // If the data is valid, we then extract the JSON-LD property aliases which we can use for property resolution later.
    let data = doc.unwrap();
    let props = data.context.get_json_ld_aliases();
    assert_eq!(props, vec![("test".to_string(), "testy-test".to_string())]);

    // If we have managed to successfully pull out the aliases, we can now attempt to deserialize the remaining property
    // values at root level. Note that the missing step here is to do the switcheroo, replacing root keys with their
    // full property values.
    let val: Result<TestProps, Error> = serde_json::from_value(serde_json::Value::Object(data.other));
    assert!(val.is_ok());
    let val_props = val.unwrap();
    assert_eq!(val_props.test, 123);
  }

  #[test]
  pub fn raw_object_parses_mastodon_object_context() {
    // This test performs the rough steps the Object::Into() method goes through when parsing ActivityPub JSON-LD
    // documents.

    // We start out with the raw JSON body, containing a context and a root key with an aliased short name key _test_,
    // resolving to the long name _testy-test_.
    let raw_json = r#"{ 
      "@context": [
        "https://www.w3.org/ns/activitystreams",
        {
            "atomUri": "ostatus:atomUri",
            "conversation": "ostatus:conversation",
            "inReplyToAtomUri": "ostatus:inReplyToAtomUri",
            "ostatus": "http://ostatus.org#",
            "sensitive": "as:sensitive",
            "toot": "http://joinmastodon.org/ns#",
            "votersCount": "toot:votersCount"
        }
      ],
      "test": 123
    }"#;

    let doc: Result<RawActivityPubDocument, Error> = serde_json::from_str(raw_json);
    assert!(doc.is_ok());

    // If the data is valid, we then extract the JSON-LD property aliases which we can use for property resolution later.
    let data = doc.unwrap();
    let props = data.context.get_json_ld_aliases();
    let eq = vec![
      ("conversation".to_string(), "ostatus:conversation".to_string()),
      ("inReplyToAtomUri".to_string(), "ostatus:inReplyToAtomUri".to_string()),
      ("votersCount".to_string(), "toot:votersCount".to_string()),
      ("atomUri".to_string(), "ostatus:atomUri".to_string()),
      ("sensitive".to_string(), "as:sensitive".to_string()),
    ];

    let a: HashSet<_> = props.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert_eq!(a, b);

    // If we have managed to successfully pull out the aliases, we can now attempt to deserialize the remaining property
    // values at root level. Note that the missing step here is to do the switcheroo, replacing root keys with their
    // full property values.
    let val: Result<TestProps, Error> = serde_json::from_value(serde_json::Value::Object(data.other));
    assert!(val.is_ok());
    let val_props = val.unwrap();
    assert_eq!(val_props.test, 123);
  }

  #[test]
  pub fn raw_object_parses_pixelfed_object_context() {
    // This test performs the rough steps the Object::Into() method goes through when parsing ActivityPub JSON-LD
    // documents.

    // We start out with the raw JSON body, containing a context and a root key with an aliased short name key _test_,
    // resolving to the long name _testy-test_.
    let raw_json = r#"{ 
      "@context": [
        "https://w3id.org/security/v1",
        "https://www.w3.org/ns/activitystreams",
        {
          "Emoji": "toot:Emoji",
          "Hashtag": "as:Hashtag",
          "announce": {
            "@id": "pixelfed:canAnnounce",
            "@type": "@id"
          },
          "capabilities": {
            "@container": "@set",
            "@id": "pixelfed:capabilities"
          },
          "commentsEnabled": {
            "@id": "pixelfed:commentsEnabled",
            "@type": "schema:Boolean"
          },
          "like": {
            "@id": "pixelfed:canLike",
            "@type": "@id"
          },
          "pixelfed": "http://pixelfed.org/ns#",
          "reply": {
            "@id": "pixelfed:canReply",
            "@type": "@id"
          },
          "schema": "http://schema.org/",
          "sensitive": "as:sensitive",
          "toot": "http://joinmastodon.org/ns#"
        }
      ],
      "test": 123
    }"#;

    let doc: Result<RawActivityPubDocument, Error> = serde_json::from_str(raw_json);
    assert!(doc.is_ok());

    // If the data is valid, we then extract the JSON-LD property aliases which we can use for property resolution later.
    let data = doc.unwrap();
    let props = data.context.get_json_ld_aliases();
    let eq = vec![
      ("Emoji".to_string(), "toot:Emoji".to_string()),
      ("announce".to_string(), "pixelfed:canAnnounce".to_string()),
      ("capabilities".to_string(), "pixelfed:capabilities".to_string()),
      ("commentsEnabled".to_string(), "pixelfed:commentsEnabled".to_string()),
      ("like".to_string(), "pixelfed:canLike".to_string()),
      ("reply".to_string(), "pixelfed:canReply".to_string()),
      ("sensitive".to_string(), "as:sensitive".to_string()),
      ("Hashtag".to_string(), "as:Hashtag".to_string()),
    ];

    let a: HashSet<_> = props.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert_eq!(a, b);

    // If we have managed to successfully pull out the aliases, we can now attempt to deserialize the remaining property
    // values at root level. Note that the missing step here is to do the switcheroo, replacing root keys with their
    // full property values.
    let val: Result<TestProps, Error> = serde_json::from_value(serde_json::Value::Object(data.other));
    assert!(val.is_ok());
    let val_props = val.unwrap();
    assert_eq!(val_props.test, 123);
  }

  #[test]
  pub fn document_parses_aliased_props() {
    let raw_json = r#"{ 
      "@context": [
        "https://www.w3.org/ns/activitystreams",
        {
            "test": "test:test"
        }
      ],
      "test": 123
    }"#;

    let raw_doc_result: Result<RawActivityPubDocument, Error> = serde_json::from_str(raw_json);
    assert!(raw_doc_result.is_ok());

    let doc_result = ActivityPubDocument::from(raw_doc_result.unwrap());
    assert!(doc_result.is_ok());

    let doc = doc_result.unwrap();
    assert!(doc.aliases.is_some());
    let aliases = doc.aliases.unwrap();
    assert!(!aliases.is_empty());
    assert_eq!(aliases["test"], "test:test");

    assert!(doc.object.extra.is_some() && doc.object.extra.as_ref().unwrap().is_object());

    let object_option = match doc.object.extra.unwrap() {
      serde_json::Value::Object(obj) => Some(obj),
      _ => None,
    };

    assert!(object_option.is_some());

    let object = object_option.unwrap();
    assert!(object.contains_key("test:test"));
    assert_eq!(
      object["test:test"],
      serde_json::Value::Number(serde_json::Number::from(123))
    );
  }

  #[test]
  pub fn document_parses_aliased_object_props() {
    let raw_json = r##"{ 
      "@context": [
        "https://www.w3.org/ns/activitystreams",
        {
            "shortcode": "orbit:shortcode"
        }
      ],
      "shortcode": "test"
    }"##;

    let raw_doc_result: Result<RawActivityPubDocument, Error> = serde_json::from_str(raw_json);
    assert!(raw_doc_result.is_ok());

    let doc_result = ActivityPubDocument::from(raw_doc_result.unwrap());
    assert!(doc_result.is_ok());

    let doc = doc_result.unwrap();
    assert!(doc.aliases.is_some());
    let aliases = doc.aliases.unwrap();
    assert!(!aliases.is_empty());
    assert_eq!(aliases["shortcode"], "orbit:shortcode");

    assert!(doc.object.orbit.is_some());
    let orbit = doc.object.orbit.unwrap();
    assert_eq!(orbit.shortcode, Some("test".to_string()));
  }

  #[test]
  pub fn document_generates_valid_wrapper() {
    temp_env::with_vars(vec![("RUN_MODE", Some("production"))], || {
      let obj = Object::builder().id(Some("test".to_string())).build();
      let doc = ActivityPubDocument::new(obj.clone());

      let mut aliases: HashMap<String, JsonLdContextMapEntry> = HashMap::new();
      aliases.insert(
        "sensitive".to_string(),
        JsonLdContextMapEntry::Alias("as:sensitive".to_string()),
      );
      aliases.insert(
        "shortcode".to_string(),
        JsonLdContextMapEntry::Alias("orbit:shortcode".to_string()),
      );
      aliases.insert(
        "summaryMd".to_string(),
        JsonLdContextMapEntry::Alias("orbit:summaryMd".to_string()),
      );
      aliases.insert(
        "orbit".to_string(),
        JsonLdContextMapEntry::Alias(format!("{}/.well-known/ns", SETTINGS.server.api_root_fqdn)),
      );

      let ctx = JsonLdContext::Multi(vec![
        JsonLdContextEntry::Uri("https://www.w3.org/ns/activitystreams".to_string()),
        JsonLdContextEntry::Uri("https://w3id.org/security/v1".to_string()),
        JsonLdContextEntry::Map(aliases),
      ]);

      assert_eq!(doc.object, obj);
      assert_eq!(doc.context, ctx);
    })
  }
}

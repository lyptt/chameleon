use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct JsonLdContextProps {
  #[serde(rename = "@id")]
  pub id: String,
  #[serde(rename = "@container")]
  pub container: Option<String>,
  #[serde(rename = "@type")]
  pub kind: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(untagged)]
pub enum JsonLdContextMapEntry {
  Alias(String),
  Props(JsonLdContextProps),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(untagged)]
pub enum JsonLdContextEntry {
  Uri(String),
  Map(HashMap<String, JsonLdContextMapEntry>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Display, Debug)]
#[serde(untagged)]
pub enum JsonLdContext {
  Uri(String),
  Map(HashMap<String, JsonLdContextMapEntry>),
  Multi(Vec<JsonLdContextEntry>),
}

impl JsonLdContext {
  fn get_json_ld_map_aliases(data: &HashMap<String, JsonLdContextMapEntry>) -> Vec<(String, String)> {
    data
      .iter()
      // We explicitly drop any external URI references to other contexts here as we can only handle
      // embedded aliases in JSON-LD documents.
      .filter(|(_, b)| match b {
        JsonLdContextMapEntry::Alias(val) => !val.starts_with("http"),
        JsonLdContextMapEntry::Props(_) => true,
      })
      .map(|(a, b)| {
        (
          a.to_owned(),
          match b {
            JsonLdContextMapEntry::Alias(val) => val.to_owned(),
            JsonLdContextMapEntry::Props(props) => props.id.to_owned(),
          },
        )
      })
      .collect()
  }

  /// Builds the series of key transformations from short keys to full keys for this JSON-LD context
  pub fn get_json_ld_aliases(&self) -> Vec<(String, String)> {
    match self {
      JsonLdContext::Uri(_) => vec![],
      JsonLdContext::Map(data) => JsonLdContext::get_json_ld_map_aliases(data),
      JsonLdContext::Multi(entries) => entries
        .iter()
        .flat_map(|v| match v {
          JsonLdContextEntry::Uri(_) => vec![],
          JsonLdContextEntry::Map(d) => JsonLdContext::get_json_ld_map_aliases(d),
        })
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::{HashMap, HashSet};

  use crate::activitypub::json_ld::{JsonLdContextEntry, JsonLdContextMapEntry};

  use super::JsonLdContext;

  #[test]
  pub fn returns_empty_vec_for_empty_context() {
    let ctx = JsonLdContext::Uri("...".to_string());
    let res = ctx.get_json_ld_aliases();
    assert!(res.is_empty());
  }

  #[test]
  pub fn returns_empty_vec_for_array_of_uris() {
    let ctx = JsonLdContext::Multi(vec![
      JsonLdContextEntry::Uri("...".to_string()),
      JsonLdContextEntry::Uri("...".to_string()),
      JsonLdContextEntry::Uri("...".to_string()),
    ]);
    let res = ctx.get_json_ld_aliases();
    assert!(res.is_empty());
  }

  #[test]
  pub fn returns_vec_for_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), JsonLdContextMapEntry::Alias("b".to_string()));
    map.insert("c".to_string(), JsonLdContextMapEntry::Alias("d".to_string()));
    let ctx = JsonLdContext::Map(map);
    let res = ctx.get_json_ld_aliases();
    let eq = vec![("a".to_string(), "b".to_string()), ("c".to_string(), "d".to_string())];

    let a: HashSet<_> = res.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert!(!res.is_empty());
    assert_eq!(a, b)
  }

  #[test]
  pub fn strips_externals_for_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), JsonLdContextMapEntry::Alias("b".to_string()));
    map.insert(
      "c".to_string(),
      JsonLdContextMapEntry::Alias("http://localhost/some-other-context".to_string()),
    );
    map.insert(
      "d".to_string(),
      JsonLdContextMapEntry::Alias("https://localhost/some-other-context".to_string()),
    );
    let ctx = JsonLdContext::Map(map);
    let res = ctx.get_json_ld_aliases();
    let eq = vec![("a".to_string(), "b".to_string())];

    let a: HashSet<_> = res.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert!(!res.is_empty());
    assert_eq!(a, b)
  }

  #[test]
  pub fn returns_vec_for_multi_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), JsonLdContextMapEntry::Alias("b".to_string()));

    let mut map_2 = HashMap::new();
    map_2.insert("c".to_string(), JsonLdContextMapEntry::Alias("d".to_string()));

    let ctx = JsonLdContext::Multi(vec![JsonLdContextEntry::Map(map), JsonLdContextEntry::Map(map_2)]);
    let res = ctx.get_json_ld_aliases();
    let eq = vec![("a".to_string(), "b".to_string()), ("c".to_string(), "d".to_string())];

    let a: HashSet<_> = res.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert!(!res.is_empty());
    assert_eq!(a, b)
  }

  #[test]
  pub fn strips_externals_for_multi_map() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), JsonLdContextMapEntry::Alias("b".to_string()));
    map.insert(
      "b".to_string(),
      JsonLdContextMapEntry::Alias("http://localhost/some-other-context".to_string()),
    );

    let mut map_2 = HashMap::new();
    map_2.insert("c".to_string(), JsonLdContextMapEntry::Alias("d".to_string()));
    map_2.insert(
      "d".to_string(),
      JsonLdContextMapEntry::Alias("https://localhost/some-other-context".to_string()),
    );

    let ctx = JsonLdContext::Multi(vec![JsonLdContextEntry::Map(map), JsonLdContextEntry::Map(map_2)]);
    let res = ctx.get_json_ld_aliases();
    let eq = vec![("a".to_string(), "b".to_string()), ("c".to_string(), "d".to_string())];

    let a: HashSet<_> = res.iter().collect();
    let b: HashSet<_> = eq.iter().collect();

    assert!(!res.is_empty());
    assert_eq!(a, b)
  }
}

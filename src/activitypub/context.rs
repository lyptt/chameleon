use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Context {
  Plain(String),
  Mapping(HashMap<String, String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContextCollection {
  Single(Context),
  Multiple(Vec<Context>),
}

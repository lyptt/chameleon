use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, PartialEq, Clone, Display, Debug)]
#[serde(untagged)]
pub enum Reference<T> {
  Embedded(Box<T>),
  Remote(String),
  Mixed(Vec<Reference<T>>),
  Map(HashMap<String, serde_json::Value>),
}

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Display, Debug, EnumString)]
#[serde(untagged)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlaceUnits {
  Cm,
  Feet,
  Inches,
  Km,
  M,
  Miles,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct PlaceProps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub accuracy: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub altitude: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub latitude: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub longitude: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub radius: Option<f32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub units: Option<PlaceUnits>,
}

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(default))]
pub struct OrbitProps {
  #[serde(
    rename(serialize = "shortcode", deserialize = "orbit:shortcode"),
    skip_serializing_if = "Option::is_none"
  )]
  pub shortcode: Option<String>,
  #[serde(
    rename(serialize = "summaryMd", deserialize = "orbit:summaryMd"),
    skip_serializing_if = "Option::is_none"
  )]
  pub summary_md: Option<String>,
}

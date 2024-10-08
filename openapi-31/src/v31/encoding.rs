// OpenAPI v3.1.0 Specification
//
// OpenAPI inside OpenAPI
//
// The version of the OpenAPI document: 3.1.0
//
// Generated by: https://openapi-generator.tech

use crate::v31;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encoding {
  #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
  pub content_type: Option<serde_json::Value>,
  #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
  pub headers: Option<std::collections::BTreeMap<String, serde_json::Value>>,
  #[serde(rename = "style", default, skip_serializing_if = "Option::is_none")]
  pub style: Option<Style>,
  #[serde(rename = "explode", default, skip_serializing_if = "Option::is_none")]
  pub explode: Option<serde_json::Value>,
  #[serde(rename = "allowReserved", default, skip_serializing_if = "Option::is_none")]
  pub allow_reserved: Option<serde_json::Value>,
}

impl Encoding {
  pub fn new() -> Encoding {
    Encoding { content_type: None, headers: None, style: None, explode: None, allow_reserved: None }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Style {
  #[serde(rename = "form")]
  Form,
  #[serde(rename = "spaceDelimited")]
  SpaceDelimited,
  #[serde(rename = "pipeDelimited")]
  PipeDelimited,
  #[serde(rename = "deepObject")]
  DeepObject,
}

impl Default for Style {
  fn default() -> Style {
    Self::Form
  }
}

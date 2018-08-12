use common::StyleProperties;
use failure::Error;
use serde_json;
use serde_yaml;

/// What format of properties keys to use and check when parsing
/// Current allowed: snake_case, camelCase, kebab-case
/// Default: camelCase
#[derive(Debug, Clone, Copy)]
pub enum PropertyCase {
  Snake,
  Kebab,
  Camel,
}

impl Default for PropertyCase {
  fn default() -> Self {
    PropertyCase::Camel
  }
}

/// From what data format to parse, currently available: JSON, YAML
/// Default: JSON
#[derive(Debug, Clone, Copy)]
pub enum ParseTarget {
  Json,
  Yaml,
}

impl Default for ParseTarget {
  fn default() -> Self {
    ParseTarget::Json
  }
}

/// One level recursion inside with:
/// * @media queries
/// * status presudo classes: :hover, :active
/// Default: Basic
#[derive(Debug, Clone, Copy)]
pub enum RecursiveType {
  Basic,
  Never,
}

impl Default for RecursiveType {
  fn default() -> Self {
    RecursiveType::Basic
  }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ParseOptions {
  pub recursive: RecursiveType,
  pub style: PropertyCase,
  pub from: ParseTarget,
}

#[derive(Debug, Fail)]
pub enum ParseError {
  #[fail(display = "invalid property '{}' case, need: {:?}", key, case)]
  InvalidKeyCase {
    case: PropertyCase,
    key: String,
  },
}

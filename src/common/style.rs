use std::collections::HashMap;
use common::StyleProperties;
use std::cell::RefCell;
use yoga::Layout;
use regex::Regex;
use std::rc::Rc;

lazy_static! {
  static ref ELEMENT_STATUS_RE: Regex = Regex::new(r"^(?P<name>[a-zA-Z_]+):(?P<status>active|hover)$").unwrap();
  static ref ELEMENT_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
}

#[derive(Debug, Clone, Default)]
pub struct Context {
  // Layout props this container
  layout: Option<Layout>,
}

#[derive(Debug, Clone, Default)]
pub struct Style {
  // Block Status properties
  pub default: Option<StyleProperties>,
  pub active: Option<StyleProperties>,
  pub hover: Option<StyleProperties>,

  // Context
  context: Context,
}

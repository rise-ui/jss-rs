use std::collections::HashMap;
use common::ElementStyle;
use std::cell::RefCell;
use regex::Regex;
use std::rc::Rc;

lazy_static! {
  static ref ELEMENT_STATUS_RE: Regex = Regex::new(r"^(?P<name>[a-zA-Z_]+):(?P<status>active|hover)$").unwrap();
  static ref ELEMENT_NAME_RE: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
}

#[derive(Debug, Clone, Default)]
pub struct Style {
  pub default: Option<ElementStyle>,
  pub hover: Option<ElementStyle>,
  pub active: Option<ElementStyle>,
}

pub type RawStylesheet = HashMap<String, ElementStyle>;
pub type Stylesheet = HashMap<String, Style>;

fn assign_to_style(entry: &mut Style, status: &str, style: ElementStyle) {
  match status {
    "default" => entry.default = Some(style),
    "active" => entry.active = Some(style),
    "hover" => entry.hover = Some(style),
    _ => {}
  }
}

fn assign_style_by_status(result: Rc<RefCell<Stylesheet>>, key: String, status: &str, style: ElementStyle) {
  let entry_opt = result.borrow().contains_key(&key);

  if entry_opt {
    assign_to_style(result.borrow_mut().get_mut(&key).unwrap(), status, style);
  } else {
    let mut entry = Style::default();
    assign_to_style(&mut entry, status, style);
    result.borrow_mut().insert(key, entry);
  }
}

// @TODO: Refactor without RefCell
pub fn get_stylesheet_from_hashmap(source: RawStylesheet) -> Stylesheet {
  let result: Rc<RefCell<Stylesheet>> = Rc::new(RefCell::new(HashMap::new()));

  for (key, style) in source {
    let is_status_name = ELEMENT_STATUS_RE.is_match(&*key);
    let is_only_name = ELEMENT_NAME_RE.is_match(&*key);

    if is_status_name {
      let status = ELEMENT_STATUS_RE.replace_all(&*key, "$status").into_owned();
      let name = ELEMENT_STATUS_RE.replace_all(&*key, "$name").into_owned();

      assign_style_by_status(result.clone(), name, &*status, style);
    } else if is_only_name {
      assign_style_by_status(result.clone(), key.to_string(), "default", style);
    }
  }

  let copy = result.borrow().clone();
  copy
}

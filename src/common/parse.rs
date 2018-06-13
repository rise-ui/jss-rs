use common::{ElementStyle, RawStylesheet, Stylesheet, get_stylesheet_from_hashmap};
use failure::Error;
use serde_json;

pub fn parse_json_style(json: String) -> Result<ElementStyle, Error> {
  let style: ElementStyle = serde_json::from_str(&json)?;
  Ok(style)
}

pub fn parse_json_stylesheet(json: String) -> Result<Stylesheet, Error> {
  let parsed: RawStylesheet = serde_json::from_str(&json)?;
  let stylesheet = get_stylesheet_from_hashmap(parsed);
  Ok(stylesheet)
}

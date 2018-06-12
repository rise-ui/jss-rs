mod style;
mod traits;

pub use self::style::ElementStyle;
pub use self::traits::*;

use failure::Error;
use serde_json;

pub fn parse_json_style(json: String) -> Result<ElementStyle, Error> {
  let style: ElementStyle = serde_json::from_str(&json)?;
  Ok(style)
}

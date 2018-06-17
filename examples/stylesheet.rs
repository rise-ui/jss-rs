extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"{
    "element": {
      "align_content": "center",
      "border_top_right_radius": 10,
      "border_top_style": "solid",
      "filter": [
        "blur(20)"
      ]
    },

    "element:hover": {
      "align_content": "flex_start",
      "background": "rgba(130,130,130,0)"
    }
  }"#;

  let result = parse_json_stylesheet(style)?;
  println!("Stylesheet: \n{:#?}", result);

  Ok(())
}

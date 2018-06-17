extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"{
    "align_content": "center",
    "border_top_right_radius": 10,
    "border_top_style": "solid",
    "filter": [
      "blur(20)"
    ]
  }"#;

  let result = parse_json_style(style)?;
  println!("Style: \n{:#?}", result);

  let (apperance, layout) = result.get_prepared_styles();
  println!("Apperance: \n{:#?}\nLayout: \n{:#?}", apperance, layout);

  Ok(())
}

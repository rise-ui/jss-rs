extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"{
    "border_top_right_radius": 10,
    "border_top_style": "solid",
    "align_content": "center",
    "border_top": 10,
    
    "filter": [
      "blur(20)"
    ],

    "transform": [
      "translate(10px,10%)",
      "rotate(40deg,15rad)"
    ]
  }"#;

  let result = parse_json_style(style)?;
  println!("Style: \n{:#?}", result);

  let (apperance, layout) = result.get_prepared_styles();
  println!("Apperance: \n{:#?}\nLayout: \n{:#?}", apperance, layout);

  Ok(())
}

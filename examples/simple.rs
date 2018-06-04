extern crate failure;
extern crate jss;

use failure::Error;
use jss::parse_json_style;

fn main() -> Result<(), Error> {
  let style = r#"{
    "align_content": "center"
  }"#;

  let result = parse_json_style(style.to_string())?;
  println!("Style: \n{:#?}", result);

  Ok(())
}

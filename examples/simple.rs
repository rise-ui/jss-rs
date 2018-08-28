extern crate failure;
extern crate jss;

use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
  let source = r#"
  {
    "border-top-right-radius": 10,
    "background": "rgb(0,0,0)",
    "border-top-width": 10
  }
  "#;

  let style = Style::parse_json_element(source, parser::RecursiveType::Basic, parser::PropertyCase::Kebab)?;
  println!("{:#?}", style);

  Ok(())
}

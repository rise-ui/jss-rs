extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

/*
@Description: this example how to parse Style struct with customized parser
It is not yet fully compatible with the jss, but supports a sufficient number of opportunities
*/

fn main() -> Result<(), Error> {
  let style = r#"---
borderTopRightRadius: 10
borderTopStyle: solid
alignContent: center
borderTop: 10
filter:
- blur(20)
transform:
- translate(10px,10%)
- rotate(40deg,15rad)
"#;

  let mut options = ParseOptions::default();
  options.from = ParseTarget::Yaml;

  let result = Style::parse_element(style, options)?;
  println!("{:#?}", result);
  Ok(())
}

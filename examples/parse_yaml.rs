extern crate failure;
extern crate jss;

use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
---
borderTopRightRadius: 10
borderTopStyle: solid
alignContent: center
borderTopWidth: 10
filter:
- blur(20)
transform:
- translate(10px,10%)
- rotate(40deg,15rad)
  "#;

    let style = Style::parse_yaml_element(source, parser::RecursiveType::Basic, parser::Case::Camel)?;
    println!("{:#?}", style);

    Ok(())
}

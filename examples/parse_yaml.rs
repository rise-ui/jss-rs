extern crate failure;
extern crate jss;

use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
---
left: $parent.width - 10
borderTopRightRadius: 10
borderTopStyle: solid
alignContent: center
borderTopWidth: 10
marginTop: 15px
marginLeft: 5%
filter:
- blur(20)
transform:
- translate(10px,10%)
- rotate(40deg,15rad)
  "#;

    let style = StyleBuilder::default().source_type(SourceFormat::Yaml).parse_from_str(source)?;
    println!("{:#?}", style);

    Ok(())
}

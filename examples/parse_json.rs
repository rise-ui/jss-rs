extern crate failure;
extern crate jss;

use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
  {
    "background": "rgba(0,0,0, 0.7)",

    "borderTopRightRadius": "10px",
    "borderTopStyle": "solid",
    "alignContent": "center",
    "borderTopWidth": 10,
    
    "filter": [
      "blur(20)"
    ],
    "transform": [
      "translate(10px,10%)",
      "rotate(40deg,15rad)"
    ]
  }
  "#;

    let style = StyleBuilder::default().parse_from_str(source)?;
    println!("{:#?}", style);

    Ok(())
}

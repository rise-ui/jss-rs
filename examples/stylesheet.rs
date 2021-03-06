extern crate failure;
extern crate jss;

use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
    {
        "my_element": {
            "borderTopRightRadius": 10,
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
        },

        "second_style": {
            "background": "rgb(0,0,0)",
            "flexDirection": "column"
        }
    }
    "#;

    let style = StylesheetBuilder::default().parse_from_str(source)?;
    println!("{:#?}", style);

    Ok(())
}

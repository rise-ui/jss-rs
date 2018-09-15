extern crate failure;
extern crate jss;

use jss::properties::{Background, Color};
use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let mut properties = Properties::default();
    let mut style = Style::default();

    // let dimensions = 
    // style.context.set_variable("layout");

    // properties.set_style(
    //     "background",
    //     Appearance::Background(Background::Color(Color::transparent())),
    // )?;

    // properties.set_style(
    //     "width",
    //     SharedUnit::CalcExpr()
    // )?;

    println!("{:#?}", properties);
    Ok(())
}

extern crate failure;
extern crate jss;

use jss::properties::{Background, Color};
use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let mut properties = Properties::default();

    properties.set_style(
        "background",
        PropertyValue::Appearance(Appearance::Background(Background::Color(Color::transparent()))),
    )?;

    println!("{:#?}", properties);
    Ok(())
}

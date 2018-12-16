extern crate failure;
extern crate eval;
extern crate jss;

use jss::types::{Properties, Style, DimensionType};
use jss::types::values::{CalcExpr, Dimensions};
use jss::properties::{Background, Color};
use jss::traits::*;
use eval::Expr;

fn main() -> Result<(), failure::Error> {
    let mut properties = Properties::default();
    let mut style = Style::default();

    let current = Dimensions::new(10., 10., 10., 10., 480., 480.);
    let parent = Dimensions::new(0., 0., 0., 0., 500., 500.);

    // Set dimensions info to style element
    style.context.set_dimension(DimensionType::Current, Some(current));
    style.context.set_dimension(DimensionType::Parent, Some(parent));

    // Set properties
    properties.set_style("background", Background::Color(Color::transparent()))?;
    // Calculated expression
    properties.set_style("height", CalcExpr(Expr::new("$parent.width + 10")))?;

    // Insert properties as new state of style
    style.states.insert("default".to_string(), properties);
    // Set enabled states
    style.enable_states(vec!["default".to_string()]);
    println!("Source: {:#?}", style);

    // Collect layout properties as FlexStyle with calculate expressions
    style.calculate_layout();
    style.calculate_appearance();
    
    println!("Computed: {:#?}", style.computed);

    Ok(())
}

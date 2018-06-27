use nom::{alpha, digit, IResult, Err, Needed};
use css_color_parser::Color as CssColor;
use properties::Color;
use std::str;

// Unit representation and parser
#[derive(Debug, Clone)]
pub struct UnitRepr<'a, 'b> {
  pub value: &'a str,
  pub unit: &'b str,
}

named!(unit_type<&[u8], &str>, alt!(
  tag!("rad") => { |_| "radians" } |
  tag!("deg") => { |_| "degrees" } |
  tag!("%")   => { |_| "percent" } |
  tag!("px")  => { |_| "point" }
));

named!(pub unit(&[u8]) -> UnitRepr, do_parse!(
  value: digit    >>
  unit: unit_type >>
  (UnitRepr {
    value: str::from_utf8(value).unwrap(),
    unit
  })
));

// Angle representation and parser
#[derive(Debug, Clone)]
pub struct AngleRepr<'a, 'b> {
  pub value: &'a str,
  pub angle: &'b str,
}

named!(angle_type<&[u8], &str>, alt!(
  tag!("rad") => { |_| "radians" } |
  tag!("deg") => { |_| "degrees" }
));

named!(pub angle(&[u8]) -> AngleRepr, do_parse!(
  value: digit      >>
  angle: angle_type >>
  (AngleRepr {
    value: str::from_utf8(value).unwrap(),
    angle
  })
));

// Stop representation and parse
#[derive(Debug, Clone)]
pub struct GradientStopRepr {
  pub color: Color,
  pub offset: f32,
}

fn take_color(input: &[u8]) -> IResult<&[u8], CssColor> {
  let color = str::from_utf8(input.clone()).unwrap();
  println!("Slice is: {}", &color);
  let color = color.parse::<CssColor>().or(Err(Err::Incomplete(Needed::Unknown)))?;
  Ok((&[], color))
}

fn prepare_gradient_stop(color: CssColor, offset: &[u8]) -> GradientStopRepr {
  let offset = str::from_utf8(offset.clone()).unwrap();
  let offset = offset.parse::<f32>().unwrap_or(0.0);
  let color = color.into();

  GradientStopRepr {
    offset,
    color,
  }
}

named!(pub gradient_stop(&[u8]) -> GradientStopRepr, do_parse!(
  color: map_res!(take_until!(" "), take_color) >>
  char!(' ')        >>
  offset: digit     >>
  char!('%')        >>
  (prepare_gradient_stop(color.1, offset))
));

// Tests of parse expressions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_stop_parse_1() {
        let stop_hex = "#FFF 10%";
        let parsed_hex = gradient_stop(stop_hex.as_bytes());
        assert!(parsed_hex.is_ok());
    }

    #[test]
    fn test_gradient_stop_parse_2() {
        let stop_rgb = "rgb(10,10,10) 10%";
        let parsed_rgb = gradient_stop(stop_rgb.as_bytes());
        assert!(parsed_rgb.is_ok());
    }

    #[test]
    fn test_gradient_stop_parse_3() {
        let stop_rgba = "rgba(10,10,10,0.1) 10%";
        let parsed_rgba = gradient_stop(stop_rgba.as_bytes());
        assert!(parsed_rgba.is_ok());
    }
}

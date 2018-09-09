use nom::{digit, IResult, Err, Needed, alpha};
use css_color_parser::Color as CssColor;
use properties::Color;
use std::str;

// Function name
named!(pub fn_name(&[u8]) -> &[u8], ws!(alpha));

// Unit representation and parser
#[derive(Debug, Clone, PartialEq)]
pub enum UnitRepr<'a, 'b> {
    Length(LengthRepr<'a, 'b>),
    Angle(AngleRepr<'a, 'b>),
}

named!(pub unit(&[u8]) -> UnitRepr, alt!(length | angle));

// Length representation and parser
#[derive(Debug, Clone, PartialEq)]
pub struct LengthRepr<'a, 'b> {
    pub value: &'a str,
    pub unit: &'b str,
}

named!(length_type<&[u8], &str>, alt!(
  tag!("%")   => { |_| "percent" } |
  tag!("n")   => { |_| "number" } | 
  tag!("px")  => { |_| "point" }
));

named!(pub length(&[u8]) -> UnitRepr, do_parse!(
  value: digit      >>
  unit: length_type >>
  (UnitRepr::Length(LengthRepr {
    value: str::from_utf8(value).unwrap(),
    unit
  }))
));

// Angle representation and parser
#[derive(Debug, Clone, PartialEq)]
pub struct AngleRepr<'a, 'b> {
    pub value: &'a str,
    pub angle: &'b str,
}

named!(angle_type<&[u8], &str>, alt!(
  tag!("rad") => { |_| "radians" } |
  tag!("deg") => { |_| "degrees" }
));

named!(pub angle(&[u8]) -> UnitRepr, do_parse!(
  value: digit      >>
  angle: angle_type >>
  (UnitRepr::Angle(AngleRepr {
    value: str::from_utf8(value).unwrap(),
    angle
  }))
));

// Stop representation and parse
#[derive(Debug, Clone, PartialEq)]
pub struct GradientStopRepr {
    pub color: Color,
    pub offset: f32,
}

fn take_color(input: &[u8]) -> IResult<&[u8], CssColor> {
    let color = str::from_utf8(input.clone()).unwrap();
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
    fn gradient_stop_parse_hex() {
        let stop_hex = "#FFF 10%";
        let parsed_hex = gradient_stop(stop_hex.as_bytes());
        let expected = GradientStopRepr {
            color: Color {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 1.0,
            },
            offset: 10.0,
        };
        assert_eq!(parsed_hex.unwrap().1, expected);
    }

    #[test]
    fn gradient_stop_parse_rgb() {
        let stop_rgb = "rgb(10,10,10) 10%";
        let parsed_rgb = gradient_stop(stop_rgb.as_bytes());

        let expected = GradientStopRepr {
            color: Color {
                red: 10,
                green: 10,
                blue: 10,
                alpha: 1.0,
            },
            offset: 10.0,
        };

        assert_eq!(parsed_rgb.unwrap().1, expected);
    }

    #[test]
    fn gradient_stop_parse_rgba() {
        let stop_rgba = "rgba(10,10,10,0.1) 10%";
        let parsed_rgba = gradient_stop(stop_rgba.as_bytes());

        let expected = GradientStopRepr {
            color: Color {
                red: 10,
                green: 10,
                blue: 10,
                alpha: 0.1,
            },
            offset: 10.0,
        };

        assert_eq!(parsed_rgba.unwrap().1, expected);
    }
}

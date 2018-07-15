use properties::parse::{UnitRepr, unit};
use nom::alpha;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct TransformFunction<'a, 'b, 'c> {
  pub args: Vec<UnitRepr<'a, 'b>>,
  pub name: &'c str,
}

named!(fn_name(&[u8]) -> &[u8], ws!(alpha));

named!(args(&[u8]) -> Vec<UnitRepr>,
  delimited!(
    char!('('),
      separated_list!(char!(','), unit),
    char!(')')
  )
);

named!(pub transform_parse(&[u8]) -> TransformFunction, do_parse!(
  name: fn_name >>
  args: args    >>
  (TransformFunction {
    name: str::from_utf8(name).unwrap(),
    args,
  })
));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn transform_function_parse() {
    use properties::parse::{LengthRepr, AngleRepr};

    let my_str = "func(10px,10deg)";
    let parsed = transform_parse(my_str.as_bytes()).expect("Can't parse transform").1;

    let expected = TransformFunction {
      args: vec![
        UnitRepr::Length(LengthRepr {
          value: "10",
          unit: "point",
        }),
        UnitRepr::Angle(AngleRepr {
          value: "10",
          angle: "degrees",
        }),
      ],
      name: "func",
    };
    assert_eq!(parsed, expected);
  }
}

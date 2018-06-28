use properties::parse::{UnitRepr, unit};
use nom::alpha;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct TransformFunction<'a, 'b, 'c> {
  pub args: Vec<UnitRepr<'a, 'b>>,
  pub name: &'c str,
}

named!(function_name(&[u8]) -> &[u8], ws!(alpha));

named!(fun_arguments(&[u8]) -> Vec<UnitRepr>,
  delimited!(
    char!('('),
      separated_list!(char!(','), unit),
    char!(')')
  )
);

named!(pub transform_parse(&[u8]) -> TransformFunction, do_parse!(
  name: function_name >>
  args: fun_arguments >>
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
    let my_str = "func(10px,10deg)";
    let parsed = transform_parse(my_str.as_bytes()).expect("Can't parse transform").1;

    let expected = TransformFunction {
      args: vec![
        UnitRepr {
          value: "10",
          unit: "point",
        },
        UnitRepr {
          value: "10",
          unit: "degrees",
        },
      ],
      name: "func",
    };
    assert_eq!(parsed, expected);
  }
}

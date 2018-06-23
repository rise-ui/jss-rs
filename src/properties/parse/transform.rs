use properties::parse::{UnitRepr, unit};
use nom::alpha;
use std::str;

#[derive(Debug, Clone)]
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

#[test]
fn test_transform_function_parse() {
  let my_str = "func(10px,10deg)";
  println!("{:?}", transform_parse(my_str.as_bytes()));
}

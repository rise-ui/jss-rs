use nom::{alpha, digit};
use std::str;

#[derive(Debug, Clone)]
pub struct FunctionRepr<'a, 'b, 'c> {
  pub args: Vec<UnitRepr<'a, 'b>>,
  pub name: &'c str,
}

#[derive(Debug, Clone)]
pub struct UnitRepr<'a, 'b> {
  pub value: &'a str,
  pub unit: &'b str,
}

named!(varname(&[u8]) -> &[u8], ws!(alpha));

named!(unit_type<&[u8], &str>, alt!(
  tag!("deg") => { |_| "degrees" } |
  tag!("%")   => { |_| "percent" } |
  tag!("px")  => { |_| "point" }
));

named!(pub unit(&[u8]) -> UnitRepr, do_parse!(
  value: digit    >>
  unit: unit_type >>
  (UnitRepr {
    value: str::from_utf8(value).unwrap(),
    unit: unit
  })
));

named!(pub fun_arguments(&[u8]) -> Vec<UnitRepr>,
  delimited!(
    char!('('),
      separated_list!(char!(','), unit),
    char!(')')
  )
);

named!(pub fun_parse(&[u8]) -> FunctionRepr, do_parse!(
  name: varname >>
  args: fun_arguments >>
  (FunctionRepr {
    name: str::from_utf8(name).unwrap(),
    args,
  })
));

#[test]
fn test_func_parse() {
  let my_str = "func(10px,10deg)";
  println!("{:?}", fun_parse(my_str.as_bytes()));
}

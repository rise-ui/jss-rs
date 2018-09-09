use properties::parse::fn_name;
use nom::float;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct FilterFunction<'a> {
    pub name: &'a str,
    pub value: f32,
}

named!(pub filter_parse(&[u8]) -> FilterFunction, do_parse!(
  name: fn_name >>
  tag!("(")     >>
  value: float  >>
  tag!(")")     >>
  (FilterFunction {
    name: str::from_utf8(name).unwrap(),
    value,
  })
));

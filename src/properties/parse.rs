use nom::{alpha, digit, IResult};
use std::str;

#[derive(Debug, Clone)]
struct FunctionRepr<'a, 'b, 'c> {
  args: Vec<UnitRepr<'a, 'b>>,
  name: &'c str,
}

#[derive(Debug, Clone)]
struct UnitRepr<'a, 'b> {
  value: &'a str,
  unit: &'b str,
}

named!(varname<&str, &str>, ws!(alpha));

named!(unit_type<&[u8], &str>, alt!(
  tag!("deg") => { |_| "degrees" } |
  tag!("%")   => { |_| "percent" } |
  tag!("px")  => { |_| "point" }
));

named!(unit<&[u8], UnitRepr>, do_parse!(
  value: digit    >>
  unit: unit_type >>
  (UnitRepr {
    value: str::from_utf8(value).unwrap(),
    unit
  })
));

named!(fun_arguments<&[u8], Vec<UnitRepr>>,
  delimited!(char!('('), separated_list!(char!(','), unit), char!(')'))
);

fn format_func(name: &'a str, args: Vec<UnitRepr>) -> FunctionRepr {
  FunctionRepr {
    name,
    args,
  }
}

named!(fun_parse<&[u8], FunctionRepr>, do_parse!(
  func_name: varname >>
  func_args: fun_arguments >>
  (format_func(func_name, func_args))
));

fn test() {
  let my_str = "(10px,10deg)";
  println!("{:?}", fun_parse(my_str.as_bytes()));
}

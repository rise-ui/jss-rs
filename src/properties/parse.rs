use nom::{alpha, digit, IResult};
use std::str;

#[derive(Debug, Clone)]
struct FunctionRepr {
  args: Vec<UnitRepr>,
  name: String,
}

#[derive(Debug, Clone)]
struct UnitRepr {
  value: String,
  unit: String,
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
    value: String::from_utf8_lossy(value).to_string(),
    unit: unit.to_string()
  })
));

named!(fun_arguments<&[u8], Vec<UnitRepr>>,
  delimited!(char!('('), separated_list!(char!(','), unit), char!(')'))
);

fn format_func(name: &str, args: Vec<UnitRepr>) -> FunctionRepr {
  FunctionRepr {
    name: name.to_string(),
    args,
  }
}

named!(fun_parse<&str, FunctionRepr>, do_parse!(
  func_name: varname >>
  func_args: fun_arguments >>
  (format_func(func_name, func_args))
));

fn test() {
  let my_str = "func(10px,10deg)";
  println!("{:?}", fun_parse(my_str));
}

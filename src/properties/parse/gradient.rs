use properties::parse::{
  GradientStopRepr,
  gradient_stop,
  AngleRepr,
  angle,
};

use nom::alpha;
use std::str;

#[derive(Debug, Clone)]
pub struct GradientFunction<'a, 'b> {
    pub stops: Vec<GradientStopRepr>,
    pub angle: AngleRepr<'a, 'b>,
}

// named!(fun_separated(&[u8]) -> GradientFunction, do_parse!(
//   angle: angle >>
//   char!(',') >>
//   stops: map_res!(take_until!(char!(b")")), separated_list!(char!(b","), gradient_stop)) >>
//   (GradientFunction {
//     angle,
//     stops
//   })
// ));

// named!(fun_arguments(&[u8]) -> (AngleRepr, Vec<GradientStopRepr>),
//   delimited!(
//     char!('('),
//       fun_separated,
//     char!(')')
//   )
// );

// // named!(pub transform_parse(&[u8]) -> GradientFunction, do_parse!(
// //   tag!("linear-gradient") >>

// //   args: fun_arguments >>
// //   (GradientFunction {

// //   })
// // ));

// #[test]
// fn test_transform_function_parse() {

// }

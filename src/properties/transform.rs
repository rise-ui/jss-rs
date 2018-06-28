use properties::{Length, Angle};
use properties::parse;
use std::str;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;

pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Transform {
  Translate((Length, Length)),
  Rotate((Angle, Angle)),
  Skew((Angle, Angle)),
  None,
}

// impl From<Transform> for String {
//   fn from(expr: Transform) -> String {
//     use self::Transform::*;

//     match expr {
//       Translate(v) => format!("translate({},{})", String::from(v.0), String::from(v.1)),
//       Rotate(v) => format!("rotate({},{})", String::from(v.0), String::from(v.1)),
//       Skew(v) => format!("skew({},{})", String::from(v.0), String::from(v.1)),
//       None => "".to_string(),
//     }
//   }
// }

// impl Serialize for Transform {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: Serializer,
//   {
//     let transform: String = self.clone().into();
//     serializer.serialize_str(&*transform)
//   }
// }

// // @TODO: adding warning or error of parse condition
// impl<'de> Deserialize<'de> for Transform {
//   fn deserialize<D>(deserializer: D) -> Result<Transform, D::Error>
//   where
//     D: Deserializer<'de>,
//   {
//     if let Value::String(transform) = Value::deserialize(deserializer)? {
//       if let Ok(parsed) = parse::transform_parse(transform.as_bytes()) {
//         let units: Vec<SharedUnit> = parsed.1.args.iter().cloned().map(Into::into).collect();
//         let name = parsed.1.name;
//         let size = units.len();

//         let params: (SharedUnit, SharedUnit) = {
//           if size > 1 {
//             (units[0], units[1])
//           } else {
//             (units[0], units[0])
//           }
//         };

//         let transform = match name {
//           "translate" => Transform::Translate(params),

//           // Extract only Angles value
//           "rotate" | "skew" => {
//             let params: (Angle, Angle) = {
//               let first = if let SharedUnit::Angle(angle) = params.0 {
//                 angle
//               } else {
//                 Angle::Degrees(0.)
//               };

//               let second = if let SharedUnit::Angle(angle) = params.1 {
//                 angle
//               } else {
//                 Angle::Degrees(0.0)
//               };

//               (first, second)
//             };

//             match name {
//               "rotate" => Transform::Rotate(params),
//               "skew" => Transform::Skew(params),
//               _ => Transform::None
//             }
//           },
//           _ => Transform::None
//         };

//         Ok(transform)
//       } else {
//         Ok(Transform::None)
//       }
//     } else {
//       Ok(Transform::None)
//     }
//   }
// }

use properties::{Length, Angle, SharedUnit, parse};
use serde::de::{self, Deserialize, Deserializer};
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

// impl From<Transform> for (String, String, String) {
//   fn from(transform: Transform) -> (String, String, String) {
//     use self::Transform::*;

//     match {
//       Translate()  
//     }
//   }
// }

// impl <'a>From<Transform> for &'a str {
//   fn from(expr: Transform) -> &'a str {
//     use self::Transform::*;

//     let expr: (String, String) = expr.into();

//     let slice = &*result;
//     slice 
//   }
// }

impl Serialize for Transform {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.clone().into())
  }
}

impl<'de> Deserialize<'de> for Transform {
  fn deserialize<D>(deserializer: D) -> Result<Transform, D::Error>
  where
    D: Deserializer<'de>,
  {
    if let Value::String(transform) = Value::deserialize(deserializer)? {
      let parsed = parse::transform_parse(transform.as_bytes()).map_err(de::Error::custom)?;
      let parsed = parsed.1;

      let name = parsed.name;
      let args = parsed.args;

      let units: Vec<SharedUnit> = args.iter().cloned().map(Into::into).collect();


      Ok(Transform::None)
    } else {
      Ok(Transform::None)
    }
  }
}
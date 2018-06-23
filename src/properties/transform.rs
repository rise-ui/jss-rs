use properties::SharedUnit;
use properties::parse;
use std::str;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;

pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Transform {
  Translate((SharedUnit, SharedUnit)),
  Rotate((SharedUnit, SharedUnit)),
  Skew((SharedUnit, SharedUnit)),
  None,
}

impl From<Transform> for String {
  fn from(expr: Transform) -> String {
    use self::Transform::*;

    match expr {
      Translate(v) => format!("translate({}, {})", String::from(v.0), String::from(v.1)),
      Rotate(v) => format!("rotate({}, {})", String::from(v.0), String::from(v.1)),
      Skew(v) => format!("skew({}, {})", String::from(v.0), String::from(v.1)),
      None => "".to_string(),
    }
  }
}

impl Serialize for Transform {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let transform: String = self.clone().into();
    serializer.serialize_str(&*transform)
  }
}

impl<'de> Deserialize<'de> for Transform {
  fn deserialize<D>(deserializer: D) -> Result<Transform, D::Error>
  where
    D: Deserializer<'de>,
  {
    if let Value::String(transform) = Value::deserialize(deserializer)? {
      if let Ok(parsed) = parse::transform_parse(transform.as_bytes()) {
        let units: Vec<SharedUnit> = parsed.1.args.iter().cloned().map(Into::into).collect();
        let size = units.len();

        let params: (SharedUnit, SharedUnit) = {
          if size > 1 {
            (units[0], units[1])
          } else {
            (units[0], units[0])
          }
        };

        let transform = match parsed.1.name {
          "translate" => Transform::Translate(params),
          "rotate" => Transform::Rotate(params),
          "skew" => Transform::Skew(params),
          _ => Transform::None,
        };

        Ok(transform)
      } else {
        Ok(Transform::None)
      }
    } else {
      Ok(Transform::None)
    }
  }
}

use properties::{Length, Angle, SharedUnit, parse};
use std::mem::discriminant as mem_entity;

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use properties::Transform;
use serde_json::Value;

fn valid_args_scheme(scheme: Vec<&str>, source: &Vec<SharedUnit>) -> bool {
  if scheme.len() == source.len() {
    let mut matches: Vec<bool> = vec![];

    for (index, unit_type) in scheme.iter().enumerate() {
      let entity = mem_entity(&source[index]);

      match unit_type.clone() {
        "length" => matches.push(mem_entity(&SharedUnit::Length(Length::Percent(1.0))) == entity),
        "angle" => matches.push(mem_entity(&SharedUnit::Angle(Angle::Degrees(1.0))) == entity),
        _ => matches.push(false),
      }
    }

    let position_unmatched = matches.iter().position(|matched| !matched);
    position_unmatched.is_none()
  } else {
    false
  }
}

fn extract_args_by_type(name: &str, source: &Vec<SharedUnit>) -> Transform {
  match name {
    "translate" => {
      let x = extract!(SharedUnit::Length(_), source[0]).unwrap_or(Length::Point(0.));
      let y = extract!(SharedUnit::Length(_), source[1]).unwrap_or(Length::Point(0.));
      Transform::Translate((x, y))
    }
    "rotate" => {
      let angle = extract!(SharedUnit::Angle(_), source[0]).unwrap_or(Angle::Degrees(0.));
      Transform::Rotate(angle)
    }
    "skew" => {
      let x = extract!(SharedUnit::Angle(_), source[0]).unwrap_or(Angle::Degrees(0.));
      let y = extract!(SharedUnit::Angle(_), source[1]).unwrap_or(Angle::Degrees(0.));
      Transform::Skew((x, y))
    }
    _ => Transform::None,
  }
}

impl From<Transform> for String {
  fn from(expr: Transform) -> String {
    use self::Transform::*;

    match expr {
      Translate((x, y)) => format!("translate({},{})", String::from(x), String::from(y)),
      Skew((x, y)) => format!("skew({},{})", String::from(x), String::from(y)),
      Rotate(rotate) => format!("rotate({})", String::from(rotate)),
      None => format!("none"),
    }
  }
}

impl Serialize for Transform {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&*String::from(self.clone()))
  }
}

impl<'de> Deserialize<'de> for Transform {
  fn deserialize<D>(deserializer: D) -> Result<Transform, D::Error>
  where
    D: Deserializer<'de>,
  {
    if let Value::String(transform) = Value::deserialize(deserializer)? {
      // @TODO: add next warning (ignore) if get errors
      let parsed = parse::transform_parse(transform.as_bytes()).map_err(de::Error::custom)?;
      let parsed = parsed.1;

      let name = parsed.name;
      let args = parsed.args;

      let units: Vec<SharedUnit> = args.iter().cloned().map(Into::into).collect();

      let is_valid = match name {
        "translate" => valid_args_scheme(vec!["length", "length"], &units),
        "skew" => valid_args_scheme(vec!["angle", "angle"], &units),
        "rotate" => valid_args_scheme(vec!["angle"], &units),
        _ => false,
      };

      if is_valid {
        Ok(extract_args_by_type(name, &units))
      } else {
        Ok(Transform::None)
      }
    } else {
      Ok(Transform::None)
    }
  }
}

// Tests of parse expressions
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validate_args_scheme() {
    let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Length(Length::Point(1.))];
    assert!(valid_args_scheme(vec!["length", "length"], &units), true);

    let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Angle(Angle::Degrees(1.))];
    assert_eq!(valid_args_scheme(vec!["angle", "length"], &units), false);
    assert!(valid_args_scheme(vec!["length", "angle"], &units), true);
  }

  #[test]
  fn extract_transform_from_args() {
    let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Length(Length::Point(1.))];
    let name = "translate";
    let expected = Transform::Translate((Length::Point(1.), Length::Point(1.)));

    let result = extract_args_by_type(name, &units);
    assert_eq!(result, expected);
  }
}

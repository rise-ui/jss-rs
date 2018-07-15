use serde::{Deserialize, Deserializer, Serializer};
use serde_json::Value;
use yoga::StyleUnit;
use std::str::FromStr;

pub fn serialize<S>(date: &Option<StyleUnit>, s: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  if let Some(ref d) = *date {
    match &d {
      StyleUnit::Percent(number) => return s.serialize_str(&*format!("{}%", number.into_inner() as i32)),
      StyleUnit::Point(number) => return s.serialize_str(&*format!("{}px", number.into_inner() as i32)),
      StyleUnit::Auto | StyleUnit::UndefinedValue => return s.serialize_str("auto"),
    }
  }

  s.serialize_none()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<StyleUnit>, D::Error>
where
  D: Deserializer<'de>,
{
  let value = Value::deserialize(deserializer)?;
  match value {
    Value::String(unit_value) => Ok(Some(parse_length(&*unit_value))),
    _ => Ok(None),
  }
}

// @note: Adapted clone of this https://doc.servo.org/style/attr/fn.parse_length.html
pub fn parse_length(mut value: &str) -> StyleUnit {
  let space_matches: &[_] = &[' ', '\t', '\n', '\u{c}', '\r'];
  value = value.trim_left_matches(space_matches);

  if value.is_empty() {
    return StyleUnit::Auto;
  }

  if value.starts_with('+') {
    value = &value[1..]
  }

  match value.chars().nth(0) {
    Some('0'...'9') => {}
    _ => return StyleUnit::Auto,
  }

  let mut end_index = value.len();
  let (mut found_full_stop, mut found_percent) = (false, false);
  for (i, ch) in value.chars().enumerate() {
    match ch {
      '0'...'9' => continue,
      '%' => {
        found_percent = true;
        end_index = i;
        break;
      }
      '.' if !found_full_stop => {
        found_full_stop = true;
        continue;
      }
      _ => {
        end_index = i;
        break;
      }
    }
  }
  value = &value[..end_index];

  if found_percent {
    let result: Result<f32, _> = FromStr::from_str(value);
    match result {
      Ok(number) => return StyleUnit::Percent(((number as f32) / 100.0).into()),
      Err(_) => return StyleUnit::Auto,
    }
  }

  let result: Result<f32, _> = FromStr::from_str(value);
  if found_percent {
    match result {
      Ok(number) => return StyleUnit::Percent(((number as f32) / 100.0).into()),
      Err(_) => return StyleUnit::Auto,
    }
  } else {
    match result {
      Ok(number) => return StyleUnit::Point(number.into()),
      Err(_) => return StyleUnit::Auto,
    }
  }
}

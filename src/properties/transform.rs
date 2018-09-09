use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use properties::parse::transform_parse;
use properties::{Length, Angle};
use serde_json::Value;

pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Transform {
    Translate((Length, Length)),
    Skew((Angle, Angle)),
    Rotate(Angle),
    None,
}

impl Transform {
    pub fn is_none(&self) -> bool {
        self.clone() == Transform::None
    }
}

impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(*self).as_str())
    }
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> Result<Transform, D::Error>
    where
        D: Deserializer<'de>,
    {
        // @TODO: next add prevalidate with `utils::valid_args_scheme`
        let transform = extract!(Value::String(_), Value::deserialize(deserializer)?)
            .and_then(|transform| Some(Transform::from(transform)))
            .unwrap_or(Transform::None);

        Ok(transform)
    }
}

impl From<String> for Transform {
    fn from(source: String) -> Transform {
        transform_parse(source.as_bytes()).and_then(|parsed| Ok(Transform::from(parsed.1))).unwrap_or(Transform::None)
    }
}

impl From<Transform> for String {
    fn from(expr: Transform) -> String {
        use self::Transform::*;
        match expr {
            Translate((x, y)) => format!("translate({},{})", String::from(x), String::from(y)),
            Skew((x, y)) => format!("skew({},{})", String::from(x), String::from(y)),
            Rotate(rotate) => format!("rotate({})", String::from(rotate)),
            None => "none".to_string(),
        }
    }
}

// Tests of parse expressions
#[cfg(test)]
mod tests {
    use properties::{Length, SharedUnit};
    use utils::extract_args_by_type;
    use super::*;

    #[test]
    fn extract_transform_from_args() {
        let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Length(Length::Point(1.))];
        let name = "translate";
        let expected = Transform::Translate((Length::Point(1.), Length::Point(1.)));

        let result = extract_args_by_type(name, &units);
        assert_eq!(result, expected);
    }
}

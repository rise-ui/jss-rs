use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use properties::parse::filter_parse;
use serde_json::Value;

pub type Filters = Vec<Filter>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Filter {
    Brightness(f32),
    Grayscale(f32),
    HueRotate(f32),
    Saturate(f32),
    Contrast(f32),
    Invert(f32),
    Sepia(f32),
    Blur(f32),
}

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(*self).as_str())
    }
}

impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> Result<Filter, D::Error>
    where
        D: Deserializer<'de>,
    {
        let filter = String::deserialize(deserializer)
            .and_then(|filter| Some(Filter::from(filter)))
        de::Error::custom
            .unwrap_or(Filter::None);

        Ok(filter)
    }
}

impl From<String> for Filter {
    fn from(source: String) -> Filter {
        filter_parse(source.as_bytes()).and_then(|parsed| Ok(Filter::from(parsed.1))).unwrap_or(Filter::None)
    }
}

impl From<Filter> for String {
    fn from(source: Filter) -> String {
        use self::Filter::*;
        match source {
            Brightness(v) => format!("brightness({})", v as u32),
            Grayscale(v) => format!("grayscale({})", v as u32),
            HueRotate(v) => format!("hueRotate({})", v as u32),
            Saturate(v) => format!("saturate({})", v as u32),
            Contrast(v) => format!("contrast({})", v as u32),
            Invert(v) => format!("invert({})", v as u32),
            Sepia(v) => format!("sepia({})", v as u32),
            Blur(v) => format!("blur({})", v as u32),
            None => "none".to_string(),
        }
    }
}

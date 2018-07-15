use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;
use regex::Regex;

#[cfg(feature = "webrender_support")]
use webrender::api;

lazy_static! {
  static ref FILTER_RE: Regex = {
    Regex::new(r"^(?P<name>brightness|grayscale|hueRotate|saturate|contrast|invert|sepia|blur)\((?P<value>\d+)\)$")
      .unwrap()
  };
}

impl Serialize for Filter {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    use self::Filter::*;

    let result = match *self {
      Brightness(value) => format!("brightness({})", value),
      Grayscale(value) => format!("grayscale({})", value),
      HueRotate(value) => format!("hueRotate({})", value),
      Saturate(value) => format!("saturate({})", value),
      Contrast(value) => format!("contrast({})", value),
      Invert(value) => format!("invert({})", value),
      Sepia(value) => format!("sepia({})", value),
      Blur(value) => format!("blur({})", value),
      None => "none".to_string(),
    };

    serializer.serialize_str(&*result)
  }
}

impl<'de> Deserialize<'de> for Filter {
  fn deserialize<D>(deserializer: D) -> Result<Filter, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = Value::deserialize(deserializer)?;
    match value {
      Value::String(filter) => Ok(filter.into()),
      _ => Ok(Filter::None),
    }
  }
}

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
  None,
}

impl Into<Filter> for String {
  fn into(self) -> Filter {
    if FILTER_RE.is_match(&*self) {
      let filter = FILTER_RE.replace_all(&*self, "$name");
      let value = match FILTER_RE.replace_all(&*self, "$value").parse::<f32>() {
        Ok(v) => v,
        Err(_) => 0.0,
      };

      match &*filter {
        "brightness" => Filter::Brightness(value),
        "grayscale" => Filter::Grayscale(value),
        "hueRotate" => Filter::HueRotate(value),
        "saturate" => Filter::Saturate(value),
        "contrast" => Filter::Contrast(value),
        "invert" => Filter::Invert(value),
        "sepia" => Filter::Sepia(value),
        "blur" => Filter::Blur(value),
        _ => Filter::None,
      }
    } else {
      Filter::None
    }
  }
}

#[cfg(feature = "webrender_support")]
impl Into<api::FilterOp> for Filter {
  fn into(self) -> api::FilterOp {
    use self::Filter::*;

    match self {
      Brightness(v) => api::FilterOp::Brightness(v),
      Grayscale(v) => api::FilterOp::Grayscale(v),
      HueRotate(v) => api::FilterOp::HueRotate(v),
      Saturate(v) => api::FilterOp::Saturate(v),
      Contrast(v) => api::FilterOp::Contrast(v),
      Invert(v) => api::FilterOp::Invert(v),
      Sepia(v) => api::FilterOp::Sepia(v),
      Blur(v) => api::FilterOp::Blur(v),
      None => api::FilterOp::Blur(0.),
    }
  }
}

use css_color_parser::Color as CssColor;
#[cfg(feature = "webrender_support")]
use webrender::api::ColorF;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}

impl Serialize for Color {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&*self.to_string())
  }
}

impl<'de> Deserialize<'de> for Color {
  fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = Value::deserialize(deserializer)?;
    match value {
      Value::String(color) => Ok(color.into()),
      _ => Ok(Color::transparent()),
    }
  }
}

impl Color {
  pub fn new(rgb: [u8; 3], alpha: f32) -> Self {
    Color {
      red: rgb[0],
      green: rgb[1],
      blue: rgb[2],
      alpha: alpha,
    }
  }

  pub fn transparent() -> Color {
    "rgba(0,0,0,0)".into()
  }

  pub fn to_string(&self) -> String {
    format!("rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha as i32)
  }
}

impl From<CssColor> for Color {
  fn from(color: CssColor) -> Color {
    Color {
      alpha: color.a,
      green: color.g,
      blue: color.b,
      red: color.r,
    }
  }
}

impl<'a> From<&'a str> for Color {
  fn from(color: &str) -> Color {
    let default_color = CssColor {
      r: 0,
      g: 0,
      b: 0,
      a: 0.0,
    };

    let color = color.parse::<CssColor>().unwrap_or(default_color);

    Color {
      alpha: color.a,
      green: color.g,
      blue: color.b,
      red: color.r,
    }
  }
}

impl From<String> for Color {
  fn from(color: String) -> Color {
    let color = &*color;
    color.into()
  }
}

#[cfg(feature = "webrender_support")]
impl From<Color> for ColorF {
  fn from(color: Color) -> ColorF {
    ColorF::new(color.red as f32 / 255., color.green as f32 / 255., color.blue as f32 / 255., color.alpha)
  }
}

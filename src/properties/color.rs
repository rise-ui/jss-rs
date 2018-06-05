#[cfg(feature = "webrender_support")]
use palette;
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
    "rgba(0,0,0,0)".to_string().into()
  }

  pub fn to_string(&self) -> String {
    format!("rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha as i32)
  }
}

impl From<String> for Color {
  fn from(color: String) -> Color {
    use css_color_parser::Color as CssColor;

    let default_color = CssColor {
      r: 0,
      g: 0,
      b: 0,
      a: 0.0,
    };

    let css_color = color.parse::<CssColor>().unwrap_or(default_color);

    Color {
      alpha: css_color.a,
      green: css_color.g,
      blue: css_color.b,
      red: css_color.r,
    }
  }
}

#[cfg(feature = "webrender_support")]
impl Into<ColorF> for Color {
  fn into(self) -> ColorF {
    use palette::rgb::Rgb;
    use palette::Alpha;

    let rgb = Alpha::<Rgb, _>::new_u8(self.red, self.green, self.blue, 255);
    let color = ColorF::new(rgb.red, rgb.green, rgb.blue, self.alpha);
    color
  }
}

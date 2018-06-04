#[cfg(feature = "webrender_support")]
use palette;
#[cfg(feature = "webrender_support")]
use webrender::api::ColorF;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: f32,
}

impl Default for Color {
  fn default() -> Self {
    Color::black()
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

  pub fn transparent() -> Self {
    Color::new([0, 0, 0], 1.0)
  }

  pub fn black() -> Self {
    Color::new([0, 0, 0], 1.0)
  }

  pub fn white() -> Self {
    Color::new([255, 255, 255], 1.0)
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

use properties::Color;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderRadius {
  pub bottom_right: i32,
  pub bottom_left: i32,
  pub top_right: i32,
  pub top_left: i32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Border {
  pub style: BorderStyle,
  pub color: Color,
  pub width: f32,
}

impl Default for Border {
  fn default() -> Border {
    Border {
      style: BorderStyle::None,
      color: Color::transparent(),
      width: 0.0,
    }
  }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderStyles {
  pub bottom: Border,
  pub right: Border,
  pub left: Border,
  pub top: Border,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BorderStyle {
  None,
  Solid,
  Double,
  Dotted,
  Dashed,
  Hidden,
  Groove,
  Ridge,
  Inset,
  Outset,
}

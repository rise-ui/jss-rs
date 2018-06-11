use properties::Color;

#[cfg(feature = "webrender_support")]
use webrender::api as wr_api;

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
}

impl Default for Border {
  fn default() -> Border {
    Border {
      style: BorderStyle::None,
      color: Color::transparent(),
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
#[serde(rename_all = "snake_case")]
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

#[cfg(feature = "webrender_support")]
impl Into<wr_api::BorderStyle> for BorderStyle {
  fn into(self) -> wr_api::BorderStyle {
    use self::BorderStyle::*;

    match self {
      None => wr_api::BorderStyle::None,
      Solid => wr_api::BorderStyle::Solid,
      Double => wr_api::BorderStyle::Double,
      Dotted => wr_api::BorderStyle::Dotted,
      Dashed => wr_api::BorderStyle::Dashed,
      Hidden => wr_api::BorderStyle::Hidden,
      Groove => wr_api::BorderStyle::Groove,
      Ridge => wr_api::BorderStyle::Ridge,
      Inset => wr_api::BorderStyle::Inset,
      Outset => wr_api::BorderStyle::Outset,
    }
  }
}

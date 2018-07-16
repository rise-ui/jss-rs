use properties::Color;

use webrender::api::{self, LayoutSize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderRadius {
  pub bottom_right: i32,
  pub bottom_left: i32,
  pub top_right: i32,
  pub top_left: i32,
}

fn get_border_radius(size: i32) -> LayoutSize {
  LayoutSize::new(size as f32, size as f32)
}

impl From<BorderRadius> for api::BorderRadius {
  fn from(radius: BorderRadius) -> api::BorderRadius {
    api::BorderRadius {
      bottom_right: get_border_radius(radius.bottom_right),
      bottom_left: get_border_radius(radius.bottom_left),
      top_right: get_border_radius(radius.top_right),
      top_left: get_border_radius(radius.top_left),
    }
  }
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

impl From<Border> for api::BorderSide {
  fn from(border: Border) -> api::BorderSide {
    api::BorderSide {
      color: border.color.into(),
      style: border.style.into(),
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

impl From<BorderStyles> for api::BorderDetails {
  fn from(border: BorderStyles) -> api::BorderDetails {
    let sides = api::NormalBorder {
      radius: api::BorderRadius::zero(),

      bottom: border.bottom.into(),
      right: border.right.into(),
      left: border.left.into(),
      top: border.top.into(),
    };

    api::BorderDetails::Normal(sides)
  }
}

impl From<BorderStyles> for api::BorderWidths {
  fn from(border: BorderStyles) -> api::BorderWidths {
    api::BorderWidths {
      bottom: border.bottom.width.clone(),
      right: border.right.width.clone(),
      left: border.left.width.clone(),
      top: border.top.width.clone(),
    }
  }
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

impl Into<api::BorderStyle> for BorderStyle {
  fn into(self) -> api::BorderStyle {
    use self::BorderStyle::*;

    match self {
      None => api::BorderStyle::None,
      Solid => api::BorderStyle::Solid,
      Double => api::BorderStyle::Double,
      Dotted => api::BorderStyle::Dotted,
      Dashed => api::BorderStyle::Dashed,
      Hidden => api::BorderStyle::Hidden,
      Groove => api::BorderStyle::Groove,
      Ridge => api::BorderStyle::Ridge,
      Inset => api::BorderStyle::Inset,
      Outset => api::BorderStyle::Outset,
    }
  }
}

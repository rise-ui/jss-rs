use properties::Color;
use yoga::StyleUnit;

use webrender::api::{
  LayoutPrimitiveInfo,
  DisplayListBuilder,
  LayoutPoint,
  LayoutSize,
  ExtendMode,
  ColorF,
  self
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Background {
  Gradient(Gradient),
  Color(Color),
}

impl Background {
  pub fn push_to_builder(
    &self,
    mut builder: DisplayListBuilder,
    background: &LayoutPrimitiveInfo,
    size: (f32, f32),
  ) -> DisplayListBuilder {
    use self::Background::*;

    match &self {
      Gradient(gradient) => {
        let from = LayoutPoint::new(size.0 * gradient.from.0, size.1 * gradient.from.1);
        let to = LayoutPoint::new(size.0 * gradient.to.0, size.1 * gradient.to.1);

        let stops: Vec<api::GradientStop> = gradient.stops.iter().cloned().map(Into::into).collect();
        let gradient = builder.create_gradient(from, to, stops, ExtendMode::Clamp);

        builder.push_gradient(&background, gradient, LayoutSize::new(size.0, size.1), LayoutSize::new(0., 0.));
      }

      Color(color) => {
        builder.push_rect(&background, ColorF::from(color.clone()));
      }
    }

    builder
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GradientStop {
  pub offset: f32,
  pub color: Color,
}

impl From<GradientStop> for api::GradientStop {
  fn from(stop: GradientStop) -> api::GradientStop {
    api::GradientStop {
      offset: stop.offset,
      color: stop.color.into(),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Gradient {
  pub stops: Vec<GradientStop>,
  // By percentage
  pub from: (f32, f32),
  pub to: (f32, f32),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BoxShadow {
  pub color: Option<Color>,
  pub horizontal: StyleUnit,
  pub vertical: StyleUnit,
  pub blur: Option<StyleUnit>,
  pub spread: Option<StyleUnit>,
  pub inset: bool,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Visibility {
  Hidden,
  Visible,
}

impl Into<bool> for Visibility {
  fn into(self) -> bool {
    match self {
      Visibility::Visible => true,
      Visibility::Hidden => false,
    }
  }
}

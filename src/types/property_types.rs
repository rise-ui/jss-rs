use ordered_float::OrderedFloat;

pub type BorderWidth = OrderedFloat<f32>;
pub type AspectRatio = OrderedFloat<f32>;
pub type FlexShrink = OrderedFloat<f32>;
pub type FlexFactor = OrderedFloat<f32>;
pub type FlexGrow = OrderedFloat<f32>;
pub type BorderColor = Color;
pub type BorderRadius = i32;

pub use yoga::{
  PositionType,
  FlexDirection,
  StyleUnit,
  FlexStyle,
  Overflow,
  Display,
  Justify,
  Align,
  Wrap
};

pub use properties::{
  BorderStyles,
  BorderStyle,
  Background,
  Transforms,
  Filters,
  Color,
};

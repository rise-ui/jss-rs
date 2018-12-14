use ordered_float::OrderedFloat;
use types::SharedUnit;

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

pub type BorderWidth = OrderedFloat<f32>;
pub type AspectRatio = OrderedFloat<f32>;
pub type FlexShrink = OrderedFloat<f32>;
pub type FlexFactor = OrderedFloat<f32>;
pub type FlexGrow = OrderedFloat<f32>;
pub type BorderRadius = SharedUnit;
pub type BorderColor = Color;

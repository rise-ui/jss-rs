//! Values that you can using in setters

use ordered_float::OrderedFloat;
use types::Appearance;

pub type BorderWidth = OrderedFloat<f32>;
pub type AspectRatio = OrderedFloat<f32>;
pub type FlexShrink = OrderedFloat<f32>;
pub type FlexFactor = OrderedFloat<f32>;
pub type FlexGrow = OrderedFloat<f32>;

pub use types::SharedUnit;

pub use properties::{
    BorderStyles,
    BorderStyle,
    Background,
    Transforms,
    Filters,
    Color,
};

pub use yoga::{
    Layout as Dimensions,
    FlexDirection,
    PositionType,
    Overflow,
    Display,
    Justify,
    Align,
    Wrap,
};

pub use yoga::StyleUnit::{
    Percent,
    Point
};

pub use self::SharedUnit::{
    StyleUnit,
    CalcExpr,
};

pub use self::Appearance::{
    BorderRadius,
    BorderColor,
};
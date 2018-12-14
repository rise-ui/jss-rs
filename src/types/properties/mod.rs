mod common;
mod repr;

pub use self::repr::get_reflect_property_type;
use self::common::*;

use std::collections::HashMap;
use types::SharedUnit;
use eval::Expr;

/// Values for appearance styles properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Appearance {
    Background(Background),
    Transforms(Transforms),
    Filters(Filters),

    BorderRadius(BorderRadius),
    BorderColor(BorderColor),
    BorderStyle(BorderStyle),

    Auto,
}

/// Values for layout styles properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Layout {
    FlexDirection(FlexDirection),
    PositionType(PositionType),
    AspectRatio(AspectRatio),
    BorderWidth(BorderWidth),
    FlexShrink(FlexShrink),
    FlexFactor(FlexFactor),
    Overflow(Overflow),
    FlexGrow(FlexGrow),
    Display(Display),
    Justify(Justify),
    Align(Align),
    Wrap(Wrap),

    // Shared Unit
    SharedUnit(SharedUnit),
}

/// Values for union of appearance and layout
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Appearance(Appearance),
    Layout(Layout),
}

/// Link type for appearance `PropertiesStore`
pub type PropertiesAppearance = PropertiesStore<Appearance>;
/// Link type for layout `PropertiesStore`
pub type PropertiesLayout = PropertiesStore<FlexStyle>;
/// Link type for calc expressions `PropertiesStore`
pub type PropertiesExpressions = PropertiesStore<Expr>;

/// Properties storage generic type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertiesStore<T>(pub HashMap<String, T>);

/// All properties of styles of different types.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Runtime Layout Calc Expressions (returned value for StyleUnit)
    pub expressions: PropertiesExpressions,

    /// Appearance properties store
    pub appearance: PropertiesAppearance,
    /// Layout properties store with yoga `FlexStyle`
    pub layout: PropertiesLayout,
}

/* _______________________Generic macro impl`s______________________ */
impl_union_property_conversion!(Appearance);
impl_union_property_conversion!(Layout);

/* ___________________Impl Traits & Conversions_____________________ */
impl<T> Default for PropertiesStore<T> {
    fn default() -> PropertiesStore<T> {
        PropertiesStore(HashMap::new())
    }
}
